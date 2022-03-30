use proc_macro2::{Delimiter, TokenStream, TokenTree};
use proc_macro_error::*;
use quote::quote;

#[proc_macro]
#[proc_macro_error]
pub fn markup(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = TokenStream::from(input).into_iter().peekable();
    let mut stack = Vec::new();
    let mut output = Vec::new();

    while let Some(token) = input.next() {
        match token {
            TokenTree::Punct(punct) => match punct.as_char() {
                '<' => {
                    let is_closing_element = match input.peek() {
                        Some(TokenTree::Punct(punct)) if punct.as_char() == '/' => {
                            // SAFETY: Guarded by above call to peek
                            input.next().unwrap();
                            true
                        }
                        _ => false,
                    };

                    let name = match input.next() {
                        Some(TokenTree::Ident(ident)) => ident,
                        Some(token) => abort!(token.span(), "unexpected token"),
                        None => abort_call_site!("unexpected end of input"),
                    };

                    let is_self_closing = match input.next() {
                        Some(TokenTree::Punct(punct)) => match punct.as_char() {
                            '>' => false,
                            '/' if !is_closing_element => {
                                match input.next() {
                                    Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {}
                                    Some(token) => abort!(token.span(), "unexpected token"),
                                    None => abort_call_site!("unexpected end of input"),
                                }
                                true
                            }
                            _ => abort!(punct.span(), "unexpected token"),
                        },
                        Some(token) => abort!(token.span(), "unexpected token"),
                        None => abort_call_site!("unexpected end of input"),
                    };

                    if !is_closing_element {
                        stack.push(name.clone());
                    } else if let Some(top) = stack.last() {
                        // Only verify the coherence of the top element on the
                        // stack with a closing element, skip over the check if
                        // the stack is empty as that error will be handled
                        // when the top element gets popped off the stack later
                        let name_str = name.to_string();
                        let top_str = top.to_string();
                        if name_str != top_str {
                            abort!(
                                name.span(), "closing element mismatch";
                                close = "found closing element {}", name_str;
                                open = top.span() => "expected {}", top_str
                            );
                        }
                    }

                    if (is_closing_element || is_self_closing) && stack.pop().is_none() {
                        abort!(name.span(), "unexpected closing element");
                    }
                }
                _ => {
                    abort!(punct.span(), "unexpected token");
                }
            },
            TokenTree::Literal(literal) => {
                let elements: Vec<_> = stack
                    .iter()
                    .map(|entry| quote! { rome_console::MarkupElement::#entry })
                    .collect();

                output.push(quote! {
                    rome_console::MarkupNode {
                        elements: &[ #( #elements ),* ],
                        content: &(#literal),
                    }
                });
            }
            TokenTree::Group(group) => match group.delimiter() {
                Delimiter::Brace => {
                    let elements: Vec<_> = stack
                        .iter()
                        .map(|entry| quote! { rome_console::MarkupElement::#entry })
                        .collect();

                    let body = group.stream();
                    output.push(quote! {
                        rome_console::MarkupNode {
                            elements: &[ #( #elements ),* ],
                            content: &(#body),
                        }
                    });
                }
                _ => abort!(group.span(), "unexpected token"),
            },
            _ => abort!(token.span(), "unexpected token"),
        }
    }

    if let Some(top) = stack.pop() {
        abort!(top.span(), "unclosed element");
    }

    quote! { rome_console::Markup(&[ #( #output ),* ]) }.into()
}
