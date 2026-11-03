//! Variable and subroutine registries with index allocation.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use super::{Diagnostics, Expr, Location, Severity, SubId, VarId};

/// Number of global variables, player variables and subroutines each.
pub(crate) const MAX_SLOTS: usize = 128;

/// Substrings the game refuses to paste in names.
const CENSORED: [&str; 5] = ["shit", "rigger", "fuck", "bong", "puss"];

/// How the program uses a variable.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct VarFlags {
    /// Chased with `Chase … Variable`.
    pub(crate) chased: bool,
    /// Used as a `For` loop variable.
    pub(crate) for_loop: bool,
    /// Read in a rule condition.
    pub(crate) in_condition: bool,
}

/// A declared global or player variable.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VarDecl {
    /// Workshop name.
    pub(crate) name: String,
    /// Explicit index, or `None` to allocate.
    pub(crate) index: Option<u8>,
    /// Initial value, emitted as an init rule by lowering.
    pub(crate) init: Option<Expr>,
    /// Compiler-created (`__like_this__`); allocated from the top.
    pub(crate) internal: bool,
    /// Usage flags.
    pub(crate) flags: VarFlags,
    /// Where it was declared.
    pub(crate) location: Option<Location>,
}

impl VarDecl {
    /// A user variable located at the caller, with no explicit index, initial value or flags.
    #[track_caller]
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            index: None,
            init: None,
            internal: false,
            flags: VarFlags::default(),
            location: Some(Location::caller()),
        }
    }
}

/// Features a subroutine body uses.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct SubUsage {
    /// Reads event-player values.
    pub(crate) event_player: bool,
    /// Reads event damage or healing values.
    pub(crate) event_damage: bool,
    /// Contains `Wait` or `Wait Until`.
    pub(crate) wait: bool,
}

/// A declared subroutine.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SubDecl {
    /// Workshop name.
    pub(crate) name: String,
    /// Explicit index, or `None` to allocate.
    pub(crate) index: Option<u8>,
    /// Compiler-created; allocated from the top.
    pub(crate) internal: bool,
    /// Subroutines it calls.
    pub(crate) calls: Vec<SubId>,
    /// What its body uses.
    pub(crate) usage: SubUsage,
    /// Where it was declared.
    pub(crate) location: Option<Location>,
}

impl SubDecl {
    /// A user subroutine located at the caller, with no explicit index, calls or flags.
    #[track_caller]
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            index: None,
            internal: false,
            calls: Vec::new(),
            usage: SubUsage::default(),
            location: Some(Location::caller()),
        }
    }
}

/// The result of a successful allocation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Allocation {
    /// Final index of each entry, in declaration order.
    pub(crate) indices: Vec<u8>,
    /// Non-fatal diagnostics.
    pub(crate) warnings: Diagnostics,
}

/// The global or player variables of a program.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Vars {
    /// Declarations; a [`VarId`] is a position in this list.
    pub(crate) entries: Vec<VarDecl>,
}

impl Vars {
    /// Adds a declaration.
    pub(crate) fn declare(&mut self, decl: VarDecl) -> VarId {
        self.entries.push(decl);
        VarId(u32::try_from(self.entries.len() - 1).unwrap_or(u32::MAX))
    }

    /// Assigns every entry its final index.
    pub(crate) fn allocate(&mut self) -> Result<Allocation, Diagnostics> {
        let slots: Vec<Slot> = self
            .entries
            .iter()
            .map(|d| Slot::new(&d.name, d.index, d.internal, d.location))
            .collect();
        let allocation = allocate(&slots, "variable", default_var_index)?;
        for (decl, &i) in self.entries.iter_mut().zip(&allocation.indices) {
            decl.index = Some(i);
        }
        Ok(allocation)
    }
}

/// The subroutines of a program.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Subs {
    /// Declarations; a [`SubId`] is a position in this list.
    pub(crate) entries: Vec<SubDecl>,
}

impl Subs {
    /// Adds a declaration.
    pub(crate) fn declare(&mut self, decl: SubDecl) -> SubId {
        self.entries.push(decl);
        SubId(u32::try_from(self.entries.len() - 1).unwrap_or(u32::MAX))
    }

    /// Assigns every entry its final index.
    pub(crate) fn allocate(&mut self) -> Result<Allocation, Diagnostics> {
        let slots: Vec<Slot> = self
            .entries
            .iter()
            .map(|d| Slot::new(&d.name, d.index, d.internal, d.location))
            .collect();
        let allocation = allocate(&slots, "subroutine", default_sub_index)?;
        for (decl, &i) in self.entries.iter_mut().zip(&allocation.indices) {
            decl.index = Some(i);
        }
        Ok(allocation)
    }

    /// Who calls whom.
    pub(crate) fn call_graph(&self) -> CallGraph {
        CallGraph {
            edges: self
                .entries
                .iter()
                .enumerate()
                .map(|(i, d)| (SubId(u32::try_from(i).unwrap_or(u32::MAX)), d.calls.clone()))
                .collect(),
        }
    }
}

/// Subroutine call edges.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct CallGraph {
    edges: BTreeMap<SubId, Vec<SubId>>,
}

impl CallGraph {
    /// Subroutines called directly by `sub`.
    pub(crate) fn callees(&self, sub: SubId) -> &[SubId] {
        self.edges.get(&sub).map_or(&[], Vec::as_slice)
    }

    /// Every subroutine reachable from `sub` through calls (including `sub` itself only if it recurses).
    pub(crate) fn reachable(&self, sub: SubId) -> BTreeSet<SubId> {
        let mut seen = BTreeSet::new();
        let mut stack = self.callees(sub).to_vec();
        while let Some(next) = stack.pop() {
            if seen.insert(next) {
                stack.extend_from_slice(self.callees(next));
            }
        }
        seen
    }

    /// Whether `sub` can end up calling itself.
    pub(crate) fn is_recursive(&self, sub: SubId) -> bool {
        self.reachable(sub).contains(&sub)
    }
}

/// What the allocator needs to know about one entry.
struct Slot<'a> {
    name: &'a str,
    index: Option<u8>,
    internal: bool,
    location: Option<Location>,
}

impl<'a> Slot<'a> {
    fn new(name: &'a str, index: Option<u8>, internal: bool, location: Option<Location>) -> Self {
        Self {
            name,
            index,
            internal,
            location,
        }
    }
}

/// The natural index of a default variable name: `A`…`Z`, `AA`…`DX`.
fn default_var_index(name: &str) -> Option<usize> {
    let bytes = name.as_bytes();
    if !(1..=2).contains(&bytes.len()) || !bytes.iter().all(u8::is_ascii_uppercase) {
        return None;
    }
    let letter = |b: u8| usize::from(b - b'A');
    let index = match *bytes {
        [a] => letter(a),
        [a, b] => 26 * (letter(a) + 1) + letter(b),
        _ => return None,
    };
    (index < MAX_SLOTS).then_some(index)
}

/// The natural index of a default subroutine name: `Sub0`…`Sub127`.
fn default_sub_index(name: &str) -> Option<usize> {
    let digits = name.strip_prefix("Sub")?;
    // Reject `Sub007` and `Sub+7`, which are different names.
    if digits.is_empty() || (digits.len() > 1 && digits.starts_with('0')) {
        return None;
    }
    if !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    digits.parse().ok().filter(|&i| i < MAX_SLOTS)
}

fn is_valid_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Which indices are taken, and which entry got which.
struct Table {
    taken: [bool; MAX_SLOTS],
    indices: Vec<Option<u8>>,
}

impl Table {
    fn claim(&mut self, entry: usize, index: usize) {
        self.taken[index] = true;
        self.indices[entry] = u8::try_from(index).ok();
    }
}

fn check_names(slots: &[Slot], kind: &str, diags: &mut Diagnostics) {
    let mut names: HashMap<&str, usize> = HashMap::new();
    for (i, slot) in slots.iter().enumerate() {
        if !is_valid_name(slot.name) {
            diags.push(
                Severity::Error,
                "invalid-name",
                format!(
                    "{kind} name `{}` must match [A-Za-z_][A-Za-z0-9_]*",
                    slot.name
                ),
                slot.location,
            );
        }
        if names.insert(slot.name, i).is_some() {
            diags.push(
                Severity::Error,
                "duplicate-name",
                format!("{kind} `{}` is declared twice", slot.name),
                slot.location,
            );
        }
        let lower = slot.name.to_ascii_lowercase();
        if CENSORED.iter().any(|bad| lower.contains(bad)) {
            diags.push(
                Severity::Warning,
                "censored-var-name",
                format!("the game refuses to paste the {kind} name `{}`", slot.name),
                slot.location,
            );
        }
    }
}

fn allocate(
    slots: &[Slot],
    kind: &str,
    default_index: fn(&str) -> Option<usize>,
) -> Result<Allocation, Diagnostics> {
    let mut diags = Diagnostics::default();
    let error = |diags: &mut Diagnostics, code, message, loc| {
        diags.push(Severity::Error, code, message, loc);
    };

    if slots.len() > MAX_SLOTS {
        error(
            &mut diags,
            "too-many",
            format!("{} {kind}s declared; the limit is {MAX_SLOTS}", slots.len()),
            None,
        );
    }

    check_names(slots, kind, &mut diags);

    let mut table = Table {
        taken: [false; MAX_SLOTS],
        indices: vec![None; slots.len()],
    };

    // Explicit indices.
    for (i, slot) in slots.iter().enumerate() {
        let Some(index) = slot.index else { continue };
        let index = usize::from(index);
        if index >= MAX_SLOTS {
            error(
                &mut diags,
                "index-out-of-range",
                format!(
                    "{kind} `{}` has index {index}; allowed are 0..={}",
                    slot.name,
                    MAX_SLOTS - 1
                ),
                slot.location,
            );
        } else if table.taken[index] {
            error(
                &mut diags,
                "duplicate-index",
                format!("{kind} index {index} is used twice (`{}`)", slot.name),
                slot.location,
            );
        } else {
            table.claim(i, index);
        }
    }
    // Default names keep their natural index when it is free.
    for (i, slot) in slots.iter().enumerate() {
        if slot.index.is_some() || slot.internal {
            continue;
        }
        if let Some(natural) = default_index(slot.name).filter(|&n| !table.taken[n]) {
            table.claim(i, natural);
        }
    }
    // Users fill from the bottom, the compiler's internals from the top.
    for (i, slot) in slots.iter().enumerate() {
        if table.indices[i].is_some() || slot.index.is_some() {
            continue;
        }
        let free = if slot.internal {
            (0..MAX_SLOTS).rev().find(|&n| !table.taken[n])
        } else {
            (0..MAX_SLOTS).find(|&n| !table.taken[n])
        };
        if let Some(n) = free {
            table.claim(i, n);
        }
    }
    let indices = table.indices;

    if diags.has_errors() {
        return Err(diags);
    }
    Ok(Allocation {
        indices: indices.into_iter().flatten().collect(),
        warnings: diags,
    })
}

#[cfg(test)]
mod tests;
