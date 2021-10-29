use proc_macro2::{Punct, Spacing};
use quote::{format_ident, quote};
use crate::codegen::{to_upper_snake_case, Result};

use super::kinds_src::KindsSrc;


pub fn generate_syntax_kinds(grammar: KindsSrc) -> Result<String> {
	let (single_byte_tokens_values, single_byte_tokens): (Vec<_>, Vec<_>) = grammar
		.punct
		.iter()
		.filter(|(token, _name)| token.len() == 1)
		.map(|(token, name)| (token.chars().next().unwrap(), format_ident!("{}", name)))
		.unzip();

	let punctuation_values = grammar.punct.iter().map(|(token, _name)| {
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
		.map(|kw| format_ident!("{}_KW", to_upper_snake_case(kw)));

	let all_keywords_values = grammar.keywords.to_vec();
	let all_keywords_idents = all_keywords_values.iter().map(|kw| format_ident!("{}", kw));
	let all_keywords = all_keywords_values
		.iter()
		.map(|name| format_ident!("{}_KW", to_upper_snake_case(name)))
		.collect::<Vec<_>>();

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

	let ast = quote! {
		#![allow(clippy::all)]
		#![allow(bad_style, missing_docs, unreachable_pub)]
		/// The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`.
		#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
		#[repr(u16)]
		pub enum SyntaxKind {
			// Technical SyntaxKinds: they appear temporally during parsing,
			// but never end up in the final tree
			#[doc(hidden)]
			TOMBSTONE,
			#[doc(hidden)]
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
		use self::SyntaxKind::*;

		impl SyntaxKind {
			pub fn is_keyword(self) -> bool {
				match self {
					#(#all_keywords)|* => true,
					_ => false,
				}
			}

			pub fn is_punct(self) -> bool {
				match self {
					#(#punctuation)|* => true,
					_ => false,
				}
			}

			pub fn is_literal(self) -> bool {
				match self {
					#(#literals)|* => true,
					_ => false,
				}
			}

			pub fn is_before_expr(self) -> bool {
				match self {
					BANG | L_PAREN | L_BRACK | L_CURLY | SEMICOLON | COMMA | COLON
					| QUESTION | PLUS2 | MINUS2 | TILDE | CASE_KW | DEFAULT_KW | DO_KW
					| ELSE_KW | RETURN_KW | THROW_KW | NEW_KW | EXTENDS_KW | YIELD_KW
					| IN_KW | TYPEOF_KW | VOID_KW | DELETE_KW | PLUSEQ | MINUSEQ
					| PIPEEQ | AMPEQ | CARETEQ | SLASHEQ | STAREQ | PERCENTEQ | AMP2
					| PIPE2 | SHLEQ | SHREQ | USHREQ | EQ | FAT_ARROW | MINUS | PLUS => true,
					_ => false,
				}
			}

			pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
				let kw = match ident {
					#(#full_keywords_values => #full_keywords,)*
					_ => return None,
				};
				Some(kw)
			}

			pub fn from_char(c: char) -> Option<SyntaxKind> {
				let tok = match c {
					#(#single_byte_tokens_values => #single_byte_tokens,)*
					_ => return None,
				};
				Some(tok)
			}

			pub fn to_string(&self) -> Option<&str> {
				let tok = match self {
					#(#punctuation => #punctuation_strings,)*
					_ => return None,
				};
				Some(tok)
			}
		}

		/// Utility macro for creating a SyntaxKind through simple macro syntax
		#[macro_export]
		macro_rules! T {
			#([#punctuation_values] => { $crate::SyntaxKind::#punctuation };)*
			#([#all_keywords_idents] => { $crate::SyntaxKind::#all_keywords };)*
			[ident] => { $crate::SyntaxKind::IDENT };
			[shebang] => { $crate::SyntaxKind::SHEBANG };
			[#] => { $crate::SyntaxKind::HASH };
		}
	};

	crate::reformat(ast)
}
