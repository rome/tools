use crate::{to_upper_snake_case, Result};
use proc_macro2::{Punct, Spacing};
use quote::{format_ident, quote};

use super::kinds_src::KindsSrc;

pub fn generate_syntax_kinds(grammar: KindsSrc) -> Result<String> {
    let punctuation_values = grammar.punct.iter().map(|(token, _name)| {
        // These tokens, when parsed to proc_macro2::TokenStream, generates a stream of bytes
        // that can't be recognized by [quote].
        // Hence, they need to be thread differently
        if "{}[]()`".contains(token) {
            let c = token.chars().next().unwrap();
            quote! { #c }
        } else {
            let cs = token.chars().map(|c| Punct::new(c, Spacing::Joint));
            quote! { #(#cs)* }
        }
    });
    let punctuation_strings = punctuation_values.clone().map(|name| name.to_string());

    let punctuation = grammar
        .punct
        .iter()
        .map(|(_token, name)| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let full_keywords_values = &grammar.keywords;
    let full_keywords = full_keywords_values
        .iter()
        .map(|kw| format_ident!("{}_KW", to_upper_snake_case(kw)))
        .collect::<Vec<_>>();

    let all_keywords_values = grammar.keywords.to_vec();
    let all_keywords_idents = all_keywords_values
        .iter()
        .map(|kw| format_ident!("{}", kw))
        .collect::<Vec<_>>();
    let all_keywords = all_keywords_values
        .iter()
        .map(|name| format_ident!("{}_KW", to_upper_snake_case(name)))
        .collect::<Vec<_>>();
    let all_keyword_strings = all_keywords_values.iter().map(|name| name.to_string());

    let literals = grammar
        .literals
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let tokens = grammar
        .tokens
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let nodes = grammar
        .nodes
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let lists = grammar
        .nodes
        .iter()
        .filter_map(|name| {
            if name.ends_with("_LIST") {
                Some(format_ident!("{}", name))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let ast = quote! {
        #![allow(clippy::all)]
        #![allow(bad_style, missing_docs, unreachable_pub)]
        /// The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        #[repr(u16)]
        pub enum JsSyntaxKind {
            // Technical SyntaxKinds: they appear temporally during parsing,
            // but never end up in the final tree
            #[doc(hidden)]
            TOMBSTONE,
            /// Marks the end of the file.May have trivia attached
            EOF,
            #(#punctuation,)*
            #(#all_keywords,)*
            #(#literals,)*
            #(#tokens,)*
            #(#nodes,)*

            // Technical kind so that we can cast from u16 safely
            #[doc(hidden)]
            __LAST,
        }
        use self::JsSyntaxKind::*;

        impl JsSyntaxKind {
            pub const fn is_punct(self) -> bool {
                match self {
                    #(#punctuation)|* => true,
                    _ => false,
                }
            }

            pub const fn is_literal(self) -> bool {
                match self {
                    #(#literals)|* => true,
                    _ => false,
                }
            }

            pub const fn is_list(self) -> bool {
                match self {
                    #(#lists)|* => true,
                    _ => false,
                }
            }

            pub const fn is_before_expr(self) -> bool {
                match self {
                    BANG | L_PAREN | L_BRACK | L_CURLY | SEMICOLON | COMMA | COLON | QUESTION | PLUS2
                    | MINUS2 | TILDE | CASE_KW | DEFAULT_KW | DO_KW | ELSE_KW | RETURN_KW | THROW_KW
                    | NEW_KW | EXTENDS_KW | YIELD_KW | IN_KW | TYPEOF_KW | VOID_KW | DELETE_KW | PLUSEQ
                    | INSTANCEOF_KW | MINUSEQ | PIPEEQ | AMPEQ | CARETEQ | SLASHEQ | STAREQ | PERCENTEQ
                    | AMP2 | PIPE2 | SHLEQ | SHREQ | USHREQ | EQ | EQ2 | EQ3 | NEQ | NEQ2 | FAT_ARROW | MINUS | PLUS | AWAIT_KW => true,
                    _ => false,
                }
            }

            pub fn from_keyword(ident: &str) -> Option<JsSyntaxKind> {
                let kw = match ident {
                    #(#full_keywords_values => #full_keywords,)*
                    _ => return None,
                };
                Some(kw)
            }

            pub const fn to_string(&self) -> Option<&'static str> {
                let tok = match self {
                    #(#punctuation => #punctuation_strings,)*
                    #(#all_keywords => #all_keyword_strings,)*
                    JS_STRING_LITERAL => "string literal",
                    _ => return None,
                };
                Some(tok)
            }
        }

        /// Utility macro for creating a SyntaxKind through simple macro syntax
        #[macro_export]
        macro_rules! T {
            #([#punctuation_values] => { $crate::JsSyntaxKind::#punctuation };)*
            #([#all_keywords_idents] => { $crate::JsSyntaxKind::#all_keywords };)*
            [ident] => { $crate::JsSyntaxKind::IDENT };
            [EOF] => { $crate::JsSyntaxKind::EOF };
            [#] => { $crate::JsSyntaxKind::HASH };
        }
    };

    xtask::reformat(ast)
}
