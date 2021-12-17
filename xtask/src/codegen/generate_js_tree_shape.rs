use super::kinds_src::AstSrc;
use crate::codegen::generate_nodes::token_kind_to_code;
use crate::codegen::kinds_src::TokenKind;
use crate::{
	codegen::{kinds_src::Field, to_upper_snake_case},
	Result,
};
use quote::{format_ident, quote};

pub fn generate_js_tree_shape(ast: &AstSrc) -> Result<String> {
	let normal_node_arms = ast.nodes.iter().map(|node| {
		let kind = format_ident!("{}", to_upper_snake_case(&node.name));
		let expected_len = node.fields.len();

		let fields = node.fields.iter().map(|field| match field {
			Field::Node { ty, .. } => {
				let ast_type_name = format_ident!("{}", ty);

				quote! {
					if slots.next().unwrap().map(#ast_type_name::can_cast) == Some(false) {
						return false;
					}
				}
			}
			Field::Token { kind, .. } => {
				let eq_kind = match kind {
					TokenKind::Single(expected) => {
						let expected_kind = token_kind_to_code(expected);
						quote! { actual == #expected_kind}
					}
					TokenKind::Many(expected) => {
						let expected_kinds = expected.iter().map(|kind| token_kind_to_code(kind));
						quote! {
							matches!(actual, #(#expected_kinds)|*)
						}
					}
				};

				quote! {
					if current_slot.is_none() {

					}

					if slots.next().unwrap().map(|actual| #eq_kind) == Some(false) {
						return false;
					}
				}
			}
		});

		quote! {
			#kind => {
				let mut current_slot = slots.next();

				#(#fields)*

				true
			}
		}
	});

	let lists = ast.lists().map(|(name, data)| {
		let element_type = format_ident!("{}", data.element_name);
		let kind = format_ident!("{}", to_upper_snake_case(name));
		if let Some(separator) = &data.separator {
			let allow_trailing = separator.allow_trailing;
			let separator_kind = token_kind_to_code(&separator.separator_token);
			quote! {
				#kind => Self::forms_separated_list_shape(#element_type::can_cast, #separator_kind, #allow_trailing, slots)
			}
		} else {
			quote! {
				#kind => Self::forms_node_list_shape(#element_type::can_cast, slots)
			}
		}
	});

	let unknown_kinds = ast
		.unknowns
		.iter()
		.map(|node| format_ident!("{}", to_upper_snake_case(node)));

	let output = quote! {
		use crate::{
			ast::*,
			T,
			JsLanguage,
			SyntaxKind::*
		};

		use rome_rowan::AstTreeShape;

		impl AstTreeShape for JsLanguage {
			fn forms_exact_shape_for(parent: Self::Kind, mut slots: impl ExactSizeIterator<Item = Option<Self::Kind>>, ) -> bool {
				match parent {
					#(#unknown_kinds)|* | ERROR => true,
					#(#normal_node_arms),*
					#(#lists),*,
					_ => unreachable!("Is {:?} a token?", parent),
				}
			}
		}
	};

	let pretty = crate::reformat(output)?;
	Ok(pretty)
}
