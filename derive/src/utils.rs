use proc_macro2::TokenStream;
use quote::ToTokens;
use std::fmt::Write;

// to get "From<usize>" instead of "From < usize >"
// this does not solve the general problem of pretty printing and will
// produce ugly results for some expressions but it is good enough
// for our usecases so I won't add a dependency on prettyplease
pub fn compact_token_stream_stringify(res: &mut String, ts: TokenStream) {
    #[derive(PartialEq, Eq)]
    enum Prev {
        Comma,
        ClosingAngleBracket,
        IdentOrLiteral,
        Other,
    }
    let mut prev = Prev::Other;
    for t in ts.to_token_stream() {
        match t {
            proc_macro2::TokenTree::Ident(ident) => {
                if matches!(
                    prev,
                    Prev::Comma
                        | Prev::IdentOrLiteral
                        | Prev::ClosingAngleBracket
                ) {
                    res.push(' ');
                }
                res.write_fmt(format_args!("{ident}")).unwrap();
                prev = Prev::IdentOrLiteral;
            }
            proc_macro2::TokenTree::Punct(punct) => {
                let p = punct.as_char();
                res.push(p);
                prev = match p {
                    ',' => Prev::Comma,
                    '>' => Prev::ClosingAngleBracket,
                    _ => Prev::Other,
                };
            }
            proc_macro2::TokenTree::Group(group) => {
                match group.delimiter() {
                    proc_macro2::Delimiter::Parenthesis => {
                        res.push('(');
                        compact_token_stream_stringify(res, group.stream());
                        res.push(')');
                    }
                    proc_macro2::Delimiter::Brace => {
                        res.push('{');
                        compact_token_stream_stringify(res, group.stream());
                        res.push('}');
                    }
                    proc_macro2::Delimiter::Bracket => {
                        res.push('[');
                        compact_token_stream_stringify(res, group.stream());
                        res.push(']');
                    }
                    proc_macro2::Delimiter::None => {
                        compact_token_stream_stringify(res, group.stream())
                    }
                }
                prev = Prev::Other;
            }
            proc_macro2::TokenTree::Literal(literal) => {
                if prev == Prev::Comma || prev == Prev::IdentOrLiteral {
                    res.push(' ');
                }
                res.write_fmt(format_args!("{literal}")).unwrap();
                prev = Prev::IdentOrLiteral;
            }
        }
    }
}

pub fn token_stream_to_compact_string(path: &TokenStream) -> String {
    let mut res = String::new();
    compact_token_stream_stringify(&mut res, path.clone());
    res
}
