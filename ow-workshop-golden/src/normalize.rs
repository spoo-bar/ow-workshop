//! Rewrites a workshop script into a canonical form so that two spellings of
//! the same script compare equal.
//!
//! The rules, applied to every script:
//! - numbers with a fraction use the shortest round-trip form (`0.50` is `0.5`);
//! - `Custom String` loses its trailing `Null` padding;
//! - quoted comment lines and `//skip:` label lines are removed;
//! - blocks left open at the end of `actions` are closed with `End;`;
//! - inside `settings`, entries are sorted and blank lines do not count;
//! - indentation is rebuilt from nesting, and blank lines outside `settings` are dropped.
//!
//! Normalizing twice gives the same result as normalizing once.

/// What to leave out of the comparison.
#[derive(Clone, Debug, Default)]
pub struct Options {
    /// Drop the `main` block (mode name and description) from `settings`.
    pub ignore_main: bool,
}

impl Options {
    /// Ignores the optional `main` block of `settings`.
    #[must_use]
    pub fn ignore_main(mut self) -> Self {
        self.ignore_main = true;
        self
    }
}

/// Normalizes with the default options.
pub fn normalize(text: &str) -> String {
    normalize_with(text, &Options::default())
}

/// Normalizes `text` according to `options`.
pub fn normalize_with(text: &str, options: &Options) -> String {
    let lines: Vec<String> = text
        .trim_start_matches('\u{feff}')
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with("//skip:") && !l.starts_with('"'))
        .map(rewrite)
        .collect();

    let Some(start) = settings_start(&lines) else {
        return layout(&lines);
    };
    let end = block_end(&lines, start + 1);
    let mut out = layout(&lines[..start]);
    out.push_str(&settings(&lines[start..=end], options));
    out.push_str(&layout(&lines[end + 1..]));
    out
}

fn settings_start(lines: &[String]) -> Option<usize> {
    lines
        .windows(2)
        .position(|w| w[0] == "settings" && w[1] == "{")
}

/// Index of the `}` closing the block whose `{` is at `open`.
fn block_end(lines: &[String], open: usize) -> usize {
    let mut depth = 0usize;
    for (i, line) in lines.iter().enumerate().skip(open) {
        if line == "{" || line.ends_with(" {") {
            depth += 1;
        } else if line == "}" {
            depth -= 1;
            if depth == 0 {
                return i;
            }
        }
    }
    lines.len() - 1
}

enum Node {
    Leaf(String),
    Block(String, Vec<Node>),
}

fn parse_nodes(lines: &[String], pos: &mut usize) -> Vec<Node> {
    let mut nodes = Vec::new();
    while *pos < lines.len() {
        let line = &lines[*pos];
        if line == "}" {
            *pos += 1;
            break;
        }
        if lines.get(*pos + 1).is_some_and(|next| next == "{") {
            let title = line.clone();
            *pos += 2;
            nodes.push(Node::Block(title, parse_nodes(lines, pos)));
        } else {
            nodes.push(Node::Leaf(line.clone()));
            *pos += 1;
        }
    }
    nodes
}

fn print_nodes(nodes: &[Node], depth: usize, options: &Options, out: &mut Vec<String>) {
    let mut printed: Vec<String> = nodes
        .iter()
        .filter(|n| {
            !(options.ignore_main && depth == 1 && matches!(n, Node::Block(t, _) if t == "main"))
        })
        .map(|node| {
            let mut buf = Vec::new();
            let pad = "\t".repeat(depth);
            match node {
                Node::Leaf(text) => buf.push(format!("{pad}{text}")),
                Node::Block(title, children) => {
                    buf.push(format!("{pad}{title}"));
                    buf.push(format!("{pad}{{"));
                    print_nodes(children, depth + 1, options, &mut buf);
                    buf.push(format!("{pad}}}"));
                }
            }
            buf.join("\n")
        })
        .collect();
    printed.sort();
    out.extend(printed);
}

fn settings(lines: &[String], options: &Options) -> String {
    let mut pos = 2; // after `settings` and `{`
    let nodes = parse_nodes(lines, &mut pos);
    let mut out = vec!["settings".to_owned(), "{".to_owned()];
    print_nodes(&nodes, 1, options, &mut out);
    out.push("}".to_owned());
    out.join("\n") + "\n"
}

const OPENERS: [&str; 4] = [
    "If(",
    "While(",
    "For Global Variable(",
    "For Player Variable(",
];

/// Re-indents with four spaces per level and closes unterminated blocks.
fn layout(lines: &[String]) -> String {
    let mut out = String::new();
    let mut push = |depth: usize, text: &str| {
        out.push_str(&"    ".repeat(depth));
        out.push_str(text);
        out.push('\n');
    };
    let (mut depth, mut open, mut in_actions) = (0usize, 0usize, false);
    for line in lines {
        if line == "}" {
            if in_actions {
                while open > 0 {
                    open -= 1;
                    push(depth + open, "End;");
                }
                in_actions = false;
            }
            depth = depth.saturating_sub(1);
            push(depth, line);
        } else if line.ends_with('{') {
            push(depth, line);
            depth += 1;
            in_actions = line == "actions {";
        } else if in_actions && (line.starts_with("Else;") || line.starts_with("Else If(")) {
            push((depth + open).saturating_sub(1), line);
        } else if in_actions && OPENERS.iter().any(|o| line.starts_with(o)) {
            push(depth + open, line);
            open += 1;
        } else if in_actions && line == "End;" {
            open = open.saturating_sub(1);
            push(depth + open, line);
        } else {
            push(depth, line);
        }
    }
    out
}

/// Applies the per-line rules: shortest numbers and `Custom String` padding.
fn rewrite(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    rewrite_chars(&chars)
}

fn is_ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn rewrite_chars(chars: &[char]) -> String {
    const CUSTOM: &str = "Custom String(";
    let mut out = String::new();
    let mut i = 0;
    let mut in_quote = false;
    while i < chars.len() {
        let c = chars[i];
        if in_quote {
            out.push(c);
            if c == '\\' && i + 1 < chars.len() {
                out.push(chars[i + 1]);
                i += 1;
            } else if c == '"' {
                in_quote = false;
            }
            i += 1;
            continue;
        }
        let prev = i.checked_sub(1).map(|p| chars[p]);
        if c == '"' {
            in_quote = true;
            out.push(c);
        } else if c.is_ascii_digit() && !prev.is_some_and(|p| is_ident(p) || p == '.') {
            let start = i;
            while chars.get(i + 1).is_some_and(char::is_ascii_digit) {
                i += 1;
            }
            let dot_digit = chars.get(i + 1) == Some(&'.')
                && chars.get(i + 2).is_some_and(char::is_ascii_digit);
            if dot_digit {
                i += 2;
                while chars.get(i + 1).is_some_and(char::is_ascii_digit) {
                    i += 1;
                }
                let text: String = chars[start..=i].iter().collect();
                match text.parse::<f64>() {
                    Ok(n) => out.push_str(&n.to_string()),
                    Err(_) => out.push_str(&text),
                }
            } else {
                out.extend(&chars[start..=i]);
            }
        } else if !prev.is_some_and(is_ident)
            && chars[i..]
                .iter()
                .take(CUSTOM.len())
                .copied()
                .eq(CUSTOM.chars())
            && let Some((args, close)) = split_args(chars, i + CUSTOM.len())
        {
            let mut args: Vec<String> = args.iter().map(|a| rewrite_chars(a)).collect();
            while args.len() > 1 && args.last().is_some_and(|a| a == "Null") {
                args.pop();
            }
            out.push_str(CUSTOM);
            out.push_str(&args.join(", "));
            out.push(')');
            i = close;
        } else {
            out.push(c);
        }
        i += 1;
    }
    out
}

/// Splits the arguments of a call whose `(` precedes `start`. Returns the
/// trimmed arguments and the index of the closing `)`.
fn split_args(chars: &[char], start: usize) -> Option<(Vec<Vec<char>>, usize)> {
    let (mut depth, mut in_quote) = (0usize, false);
    let mut args = Vec::new();
    let mut current: Vec<char> = Vec::new();
    let mut i = start;
    while i < chars.len() {
        let c = chars[i];
        if in_quote {
            current.push(c);
            if c == '\\' && i + 1 < chars.len() {
                current.push(chars[i + 1]);
                i += 1;
            } else if c == '"' {
                in_quote = false;
            }
        } else {
            match c {
                '"' => {
                    in_quote = true;
                    current.push(c);
                }
                '(' => {
                    depth += 1;
                    current.push(c);
                }
                ')' if depth == 0 => {
                    args.push(trim(&current));
                    return Some((args, i));
                }
                ')' => {
                    depth -= 1;
                    current.push(c);
                }
                ',' if depth == 0 => args.push(trim(&std::mem::take(&mut current))),
                _ => current.push(c),
            }
        }
        i += 1;
    }
    None
}

fn trim(chars: &[char]) -> Vec<char> {
    let text: String = chars.iter().collect();
    text.trim().chars().collect()
}
