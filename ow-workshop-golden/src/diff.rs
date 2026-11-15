//! A small line diff for failure messages.

const CONTEXT: usize = 3;

enum Op<'a> {
    Same(&'a str),
    Del(&'a str),
    Add(&'a str),
}

/// Renders the differences between two texts, with a few lines of context.
pub(crate) fn lines(old: &str, new: &str) -> String {
    let old: Vec<&str> = old.lines().collect();
    let new: Vec<&str> = new.lines().collect();
    let ops = ops(&old, &new);
    let mut keep = vec![false; ops.len()];
    for (i, op) in ops.iter().enumerate() {
        if !matches!(op, Op::Same(_)) {
            let from = i.saturating_sub(CONTEXT);
            let to = (i + CONTEXT + 1).min(ops.len());
            keep[from..to].fill(true);
        }
    }
    let mut out = String::new();
    let mut skipping = false;
    for (op, keep) in ops.iter().zip(keep) {
        if !keep {
            if !skipping {
                out.push_str("  ...\n");
            }
            skipping = true;
            continue;
        }
        skipping = false;
        let (mark, text) = match op {
            Op::Same(t) => (' ', t),
            Op::Del(t) => ('-', t),
            Op::Add(t) => ('+', t),
        };
        out.push(mark);
        out.push(' ');
        out.push_str(text);
        out.push('\n');
    }
    out
}

fn ops<'a>(old: &[&'a str], new: &[&'a str]) -> Vec<Op<'a>> {
    let prefix = old.iter().zip(new).take_while(|(a, b)| a == b).count();
    let suffix = old[prefix..]
        .iter()
        .rev()
        .zip(new[prefix..].iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    let old_mid = &old[prefix..old.len() - suffix];
    let new_mid = &new[prefix..new.len() - suffix];

    // Longest common subsequence table over the differing middle only.
    let width = new_mid.len() + 1;
    let mut table = vec![0u32; (old_mid.len() + 1) * width];
    for i in (0..old_mid.len()).rev() {
        for j in (0..new_mid.len()).rev() {
            table[i * width + j] = if old_mid[i] == new_mid[j] {
                table[(i + 1) * width + j + 1] + 1
            } else {
                table[(i + 1) * width + j].max(table[i * width + j + 1])
            };
        }
    }

    let mut ops: Vec<Op> = old[..prefix].iter().map(|t| Op::Same(t)).collect();
    let (mut i, mut j) = (0, 0);
    while i < old_mid.len() && j < new_mid.len() {
        if old_mid[i] == new_mid[j] {
            ops.push(Op::Same(old_mid[i]));
            i += 1;
            j += 1;
        } else if table[(i + 1) * width + j] >= table[i * width + j + 1] {
            ops.push(Op::Del(old_mid[i]));
            i += 1;
        } else {
            ops.push(Op::Add(new_mid[j]));
            j += 1;
        }
    }
    ops.extend(old_mid[i..].iter().map(|t| Op::Del(t)));
    ops.extend(new_mid[j..].iter().map(|t| Op::Add(t)));
    ops.extend(old[old.len() - suffix..].iter().map(|t| Op::Same(t)));
    ops
}
