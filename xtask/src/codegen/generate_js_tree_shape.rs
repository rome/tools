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

		let fields = node.fields.iter().map(|field| {
			let valid = match field {
				Field::Node { ty, .. } => {
					let ast_type_name = format_ident!("{}", ty);

					quote! {
						#ast_type_name::can_cast(*current)
					}
				}
				Field::Token { kind, .. } => match kind {
					TokenKind::Single(expected) => {
						let expected_kind = token_kind_to_code(expected);
						quote! { *current == #expected_kind}
					}
					TokenKind::Many(expected) => {
						let expected_kinds = expected.iter().map(|kind| token_kind_to_code(kind));
						quote! {
							matches!(*current, #(#expected_kinds)|*)
						}
					}
				},
			};

			let optional = matches!(
				field,
				Field::Node { optional: true, .. } | Field::Token { optional: true, .. }
			);

			let invalid_case = if optional {
				quote! {
					shape.empty()
				}
			} else {
				quote! {
					drop(kinds);
					return receiver(Err(slots));
				}
			};

			quote! {
				if let Some(current) = &current_kind {
						if #valid {
							shape.occupied();
							current_kind = kinds.next();
						} else {
							#invalid_case
						}
					} else {
						#invalid_case
					}
			}
		});

		quote! {
			#kind => {
				if actual_len > #expected_len {
					return receiver(Err(slots));
				}

				let mut shape = NodeShapCommands::<#expected_len>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();

				#(#fields)*

				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}

				drop(kinds);
				receiver(Ok(NodeShape::Normal { commands: shape.as_slice(), parsed_elements: slots }))
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
				#kind => receiver(if Self::forms_separated_list_shape(#element_type::can_cast, #separator_kind, #allow_trailing, slots.kinds()) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				})
			}
		} else {
			quote! {
				#kind => receiver(if Self::forms_node_list_shape(#element_type::can_cast, slots.kinds()) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				})
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

		use rome_rowan::{AstTreeShape, ParsedElements, NodeShapeCommand, NodeShapCommands, NodeShape};

		impl AstTreeShape for JsLanguage {
			fn forms_exact_shape_for<F, R>(
				parent: Self::Kind,
				slots: ParsedElements<Self>,
				receiver: F,
			) -> R
			where
				F: FnOnce(Result<NodeShape<'_, Self>, ParsedElements<'_, Self>>) -> R {
				let actual_len = slots.len();
				match parent {
					#(#unknown_kinds)|* | ERROR => {
						receiver(Ok(NodeShape::List(slots)))
					},
					#(#normal_node_arms),*,
					#(#lists),*,
					_ => unreachable!("Is {:?} a token?", parent),
				}
			}
		}
	};

	let pretty = crate::reformat(output)?;
	Ok(pretty)
}
