//! Generates `values.rs`: one constructor per workshop value, from the
//! value table and signatures in `ow-workshop-data`.

use std::fmt::Write as _;
use std::{env, fs, path::PathBuf};

use ow_workshop_data::{Ty, ValueId};

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    let mut out = String::new();
    for &id in ValueId::ALL {
        constructor(&mut out, id);
    }
    out.push_str("/// Every value with a constructor in this module.\n");
    out.push_str("#[doc(hidden)]\npub const COVERED: &[ValueId] = &[\n");
    for id in ValueId::ALL {
        let _ = writeln!(out, "    ValueId::{id:?},");
    }
    out.push_str("];\n");
    let path = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR")).join("values.rs");
    fs::write(path, out).expect("write values.rs");
}

/// A parameter of a generated constructor.
struct Param {
    name: String,
    ty: String,
    /// The expression turning the parameter into a `Value`.
    value: String,
}

fn constructor(out: &mut String, id: ValueId) {
    let name = fn_name(id.name());
    let doc = doc(id);
    let (params, body) = match id {
        ValueId::True | ValueId::False => (
            String::new(),
            format!("Value::Bool({})", id == ValueId::True),
        ),
        ValueId::Number => ("number: f64".to_owned(), "Value::Number(number)".to_owned()),
        ValueId::Compare => (
            "lhs: impl Into<Value>, op: CompareOp, rhs: impl Into<Value>".to_owned(),
            "lhs.into().compare(op, rhs)".to_owned(),
        ),
        ValueId::CustomString => (
            "text: impl Into<String>, args: impl IntoIterator<Item = Value>".to_owned(),
            "Value::string(text, args)".to_owned(),
        ),
        ValueId::GlobalVar => ("variable: &Global".to_owned(), "variable.into()".to_owned()),
        ValueId::PlayerVar => (
            "player: impl Into<Value>, variable: &PlayerVar".to_owned(),
            "variable.of(player.into())".to_owned(),
        ),
        _ => generic(id),
    };
    let _ = writeln!(
        out,
        "/// {doc}\npub fn {name}({params}) -> Value {{\n    {body}\n}}\n"
    );
}

/// Parameters and body for a value written as `Name(args...)`.
fn generic(id: ValueId) -> (String, String) {
    let mut params = Vec::new();
    let mut variadic = None;
    for arg in id.signature().args {
        if arg.name.starts_with('{') || arg.name.starts_with('[') {
            let name = if arg.name.starts_with('{') {
                "args"
            } else {
                "items"
            };
            variadic.get_or_insert(name);
            continue;
        }
        let mut name = snake(arg.name);
        if params.iter().any(|p: &Param| p.name == name) {
            name = format!("{name}_2");
        }
        if matches!(name.as_str(), "if" | "else" | "type" | "default") {
            name.push('_');
        }
        let (ty, value) = param_type(arg.ty, &name);
        params.push(Param { name, ty, value });
    }
    let mut sig: Vec<String> = params
        .iter()
        .map(|p| format!("{}: {}", p.name, p.ty))
        .collect();
    let values: Vec<&str> = params.iter().map(|p| p.value.as_str()).collect();
    let body = match (variadic, values.is_empty()) {
        (Some(rest), _) => {
            sig.push(format!("{rest}: impl IntoIterator<Item = Value>"));
            format!(
                "Value::call(ValueId::{id:?}, [{}].into_iter().chain({rest}))",
                values.join(", ")
            )
        }
        (None, true) => format!("Value::of(ValueId::{id:?})"),
        (None, false) => format!("Value::call(ValueId::{id:?}, [{}])", values.join(", ")),
    };
    (sig.join(", "), body)
}

/// The Rust type of an argument, and how it becomes a `Value`.
fn param_type(ty: Ty, name: &str) -> (String, String) {
    let literal = match ty {
        Ty::ButtonLiteral => Some("Button"),
        Ty::ColorLiteral => Some("Color"),
        Ty::GamemodeLiteral => Some("Gamemode"),
        Ty::HeroLiteral => Some("Hero"),
        Ty::MapLiteral => Some("Map"),
        Ty::TeamLiteral => Some("Team"),
        _ => None,
    };
    if let Some(enum_name) = literal {
        return (
            format!("data::{enum_name}"),
            format!("Value::Constant({name}.name())"),
        );
    }
    let constant = match ty {
        Ty::BarrierLos => "BarrierLos",
        Ty::Comms => "Comms",
        Ty::CompareOp => "CompareOp",
        Ty::Health => "Health",
        Ty::HeroStat => "HeroStat",
        Ty::Icon => "Icon",
        Ty::LosCheck => "LosCheck",
        Ty::Rounding => "Rounding",
        Ty::Stat => "Stat",
        Ty::Status => "Status",
        Ty::Transform => "Transform",
        _ => return ("impl Into<Value>".to_owned(), format!("{name}.into()")),
    };
    (format!("data::{constant}"), format!("{name}.into()"))
}

/// The one-line doc: the workshop text with argument names.
fn doc(id: ValueId) -> String {
    let args: Vec<&str> = id.signature().args.iter().map(|a| a.name).collect();
    if args.is_empty() {
        format!("`{}`.", id.name())
    } else {
        format!("`{}({})`.", id.name(), args.join(", "))
    }
}

/// `Is Game In Progress` becomes `is_game_in_progress`; keywords get a `_` suffix.
fn fn_name(workshop_name: &str) -> String {
    let mut name = String::new();
    for word in workshop_name
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
    {
        if !name.is_empty() {
            name.push('_');
        }
        name.push_str(&word.to_ascii_lowercase());
    }
    if matches!(name.as_str(), "true" | "false") {
        name.push('_');
    }
    name
}

/// `playersToInclude` becomes `players_to_include`.
fn snake(camel: &str) -> String {
    let mut name = String::new();
    for c in camel.chars() {
        if c.is_ascii_uppercase() {
            name.push('_');
        }
        name.push(c.to_ascii_lowercase());
    }
    name
}
