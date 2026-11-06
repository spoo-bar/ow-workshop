//! `ws_fmt!`: `format!`-like string templates.

use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, Ident, LitStr, Token};

/// One argument after the format string: `expr` or `name = expr`.
struct Arg {
    name: Option<Ident>,
    expr: Expr,
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let named = input.peek(Ident) && input.peek2(Token![=]) && !input.peek2(Token![==]);
        let name = if named {
            let name = input.parse()?;
            input.parse::<Token![=]>()?;
            Some(name)
        } else {
            None
        };
        Ok(Self {
            name,
            expr: input.parse()?,
        })
    }
}

struct Input {
    template: LitStr,
    args: Vec<Arg>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let template = input.parse()?;
        let mut args = Vec::new();
        if input.parse::<Option<Token![,]>>()?.is_some() {
            args = Punctuated::<Arg, Token![,]>::parse_terminated(input)?
                .into_iter()
                .collect();
        }
        Ok(Self { template, args })
    }
}

/// A piece of the parsed template.
enum Piece {
    Text(String),
    Arg(usize),
}

/// How a placeholder names its argument.
enum Reference {
    Next,
    Index(usize),
    Name(String),
}

/// Splits the template into text and placeholder references.
fn parse_template(text: &str, span: Span) -> syn::Result<Vec<(Option<Reference>, String)>> {
    let err = |msg: &str| syn::Error::new(span, msg);
    let mut out = Vec::new();
    let mut literal = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' if chars.peek() == Some(&'{') => {
                chars.next();
                literal.push('{');
            }
            '}' if chars.peek() == Some(&'}') => {
                chars.next();
                literal.push('}');
            }
            '}' => {
                return Err(err(
                    "unmatched `}` in the format string; use `}}` for a brace",
                ));
            }
            '{' => {
                let mut inner = String::new();
                loop {
                    match chars.next() {
                        Some('}') => break,
                        Some(c) => inner.push(c),
                        None => return Err(err("unclosed `{` in the format string")),
                    }
                }
                if inner.contains(':') {
                    return Err(err("format specs are not supported in `ws_fmt!`"));
                }
                let inner = inner.trim();
                let reference = if inner.is_empty() {
                    Reference::Next
                } else if inner.chars().all(|c| c.is_ascii_digit()) {
                    Reference::Index(
                        inner
                            .parse()
                            .map_err(|_| err("placeholder index out of range"))?,
                    )
                } else {
                    Reference::Name(inner.to_owned())
                };
                out.push((Some(reference), std::mem::take(&mut literal)));
            }
            c => literal.push(c),
        }
    }
    out.push((None, literal));
    Ok(out)
}

pub(crate) fn expand(input: TokenStream) -> TokenStream {
    match try_expand(input) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
}

fn try_expand(input: TokenStream) -> syn::Result<TokenStream> {
    let Input { template, args } = syn::parse2(input)?;
    let span = template.span();
    let parsed = parse_template(&template.value(), span)?;

    let positional: Vec<&Arg> = args.iter().filter(|a| a.name.is_none()).collect();
    let mut exprs: Vec<TokenStream> = positional
        .iter()
        .map(|a| a.expr.to_token_stream())
        .collect();
    let mut names: HashMap<String, usize> = HashMap::new();
    for arg in args.iter().filter(|a| a.name.is_some()) {
        let name = arg
            .name
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default();
        names.insert(name, exprs.len());
        exprs.push(arg.expr.to_token_stream());
    }
    let explicit = exprs.len();
    let mut used = vec![false; explicit];

    let (mut implicit, mut numbered) = (false, false);
    let mut next = 0;
    let mut pieces = Vec::new();
    for (reference, text) in parsed {
        if !text.is_empty() {
            match pieces.last_mut() {
                Some(Piece::Text(prev)) => prev.push_str(&text),
                _ => pieces.push(Piece::Text(text)),
            }
        }
        let Some(reference) = reference else { continue };
        let is_positional = !matches!(reference, Reference::Name(_));
        let index = match reference {
            Reference::Next => {
                implicit = true;
                next += 1;
                next - 1
            }
            Reference::Index(n) => {
                numbered = true;
                n
            }
            Reference::Name(name) => {
                if let Some(&i) = names.get(&name) {
                    i
                } else {
                    let ident = syn::parse_str::<Ident>(&name).map_err(|_| {
                        syn::Error::new(span, format!("`{name}` is not an identifier"))
                    })?;
                    let ident = Ident::new(&ident.to_string(), span);
                    names.insert(name, exprs.len());
                    exprs.push(quote!(#ident));
                    exprs.len() - 1
                }
            }
        };
        if implicit && numbered {
            return Err(syn::Error::new(
                span,
                "cannot mix `{}` and `{0}` placeholders in one string",
            ));
        }
        if is_positional && index >= positional.len() {
            let n = positional.len();
            return Err(syn::Error::new(
                span,
                format!(
                    "invalid reference to positional argument {index} (there {} {n} argument{})",
                    if n == 1 { "is" } else { "are" },
                    if n == 1 { "" } else { "s" },
                ),
            ));
        }
        if index < used.len() {
            used[index] = true;
        }
        pieces.push(Piece::Arg(index));
    }

    let sources: Vec<&Arg> = positional
        .iter()
        .copied()
        .chain(args.iter().filter(|a| a.name.is_some()))
        .collect();
    for (arg, was_used) in sources.iter().zip(&used) {
        if !was_used {
            return Err(syn::Error::new_spanned(
                &arg.expr,
                "argument never used in the format string",
            ));
        }
    }

    Ok(emit(&pieces, &exprs))
}

/// The call that builds the template at run time.
fn emit(pieces: &[Piece], exprs: &[TokenStream]) -> TokenStream {
    let pieces = pieces.iter().map(|p| match p {
        Piece::Text(t) => quote!(::ow_workshop::__private::Piece::Text(#t)),
        Piece::Arg(i) => quote!(::ow_workshop::__private::Piece::Arg(#i)),
    });
    let exprs = exprs
        .iter()
        .map(|e| quote!(::ow_workshop::IntoExpr::<::ow_workshop::Any>::into_expr(#e)));
    quote! {
        ::ow_workshop::__private::template(&[#(#pieces),*], ::std::vec![#(#exprs),*])
    }
}
