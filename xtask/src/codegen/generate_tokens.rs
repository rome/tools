use quote::{format_ident, quote};
use crate::codegen::{to_upper_snake_case, Result};
use super::kinds_src::AstSrc;

pub fn generate_tokens(grammar: &AstSrc) -> Result<String> {
	let tokens = grammar.tokens.iter().map(|token| {
		let name = format_ident!("{}", token);
		let kind = format_ident!("{}", to_upper_snake_case(token));
		quote! {
			#[derive(Debug, Clone, PartialEq, Eq, Hash)]
			pub struct #name {
				pub(crate) syntax: SyntaxToken,
			}
			impl std::fmt::Display for #name {
				fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
					std::fmt::Display::fmt(&self.syntax, f)
				}
			}
			impl AstToken for #name {
				fn can_cast(kind: SyntaxKind) -> bool { kind == #kind }
				fn cast(syntax: SyntaxToken) -> Option<Self> {
					if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
				}
				fn syntax(&self) -> &SyntaxToken { &self.syntax }
			}
		}
	});

	let pretty = crate::reformat(quote! {
		use crate::{
			ast::AstToken,
			SyntaxKind::{self, *},
			SyntaxToken,
		};

		#(#tokens)*
	})?
	.replace("#[derive", "\n#[derive");
	Ok(pretty)
}
