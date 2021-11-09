use super::kinds_src::{AstSrc, KindsSrc};
use crate::codegen::Result;
use quote::{format_ident, quote};

pub fn generate_utils(_ast: &AstSrc, grammar: KindsSrc) -> Result<String> {
	let matches: Vec<_> = grammar
		.nodes
		.iter()
		.filter_map(|node| {
			if !node.contains("UNKNOWN") || node.contains("TS_UNKNOWN") {
				None
			} else {
				let node = format_ident!("{}", node);
				Some(quote! {
					#node
				})
			}
		})
		.collect();

	let concat_matches = quote! {
				#(#matches)|*

	};
	let ast = quote! {
		use crate::SyntaxKind::{self, *};

		pub fn is_unknown_kind(kind: SyntaxKind) -> bool {
			matches!(
				kind,
				#concat_matches
			)
		}
	};

	let pretty = crate::reformat(ast.to_string())?;
	Ok(pretty)
}
