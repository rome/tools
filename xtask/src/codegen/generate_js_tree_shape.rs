use super::kinds_src::AstSrc;
use crate::codegen::generate_nodes::token_kind_to_code;
use crate::codegen::kinds_src::TokenKind;
use crate::{
	codegen::{kinds_src::Field, to_upper_snake_case},
	Result,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate_js_tree_shape(ast: &AstSrc) -> Result<String> {
	let validate_slot = generate_validate_slot(ast);
	let validate_end = generate_validate_end(ast);

	let output = quote! {
		use crate::{
			ast::*,
			T,
			JsLanguage,
			SyntaxKind::{*}
		};

		use rome_rowan::AstTreeShape;

		impl AstTreeShape for JsLanguage {
			#validate_slot

			#validate_end
		}
	};

	let pretty = crate::reformat(output)?;
	Ok(pretty)
}

fn generate_validate_slot(ast: &AstSrc) -> TokenStream {
	let normal_node_arms = ast.nodes.iter().map(|node| {
		let kind = format_ident!("{}", to_upper_snake_case(&node.name));

		let fields: Vec<_> = node
			.fields
			.iter()
			.enumerate()
			.map(|(index, field)| {
				let arm_body = match field {
					Field::Node { ty, .. } => {
						let ast_type_name = format_ident!("{}", ty);

						quote! {
							value.is_none() || #ast_type_name::can_cast(value.unwrap())
						}
					}

					Field::Token { kind, .. } => {
						let matches = match kind {
							TokenKind::Single(expected) => {
								let expected_kind = token_kind_to_code(expected);
								quote! { value.unwrap() == #expected_kind}
							}
							TokenKind::Many(expected) => {
								let expected_kinds =
									expected.iter().map(|kind| token_kind_to_code(kind));
								quote! {
									matches!(value.unwrap(), #(#expected_kinds)|*)
								}
							}
						};

						quote! {
							value.is_none() || #matches
						}
					}
				};

				quote! {
					#index => #arm_body
				}
			})
			.collect();

		if fields.is_empty() {
			quote! {
				#kind => false
			}
		} else {
			quote! {
				#kind => {
					match index {
						#(#fields),*,
						_ => false
					}
				}
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
				#kind => {
					let expects_element = index % 2 == 0;

					if expects_element {
						value.is_none() ||  #element_type::can_cast(value.unwrap())
					} else {
						value.is_none() || value.unwrap() == #separator_kind
					}
				}
			}
		} else {
			quote! {
				#kind => value.is_none() || #element_type::can_cast(value.unwrap())
			}
		}
	});

	let unknown_kinds = ast
		.unknowns
		.iter()
		.map(|node| format_ident!("{}", to_upper_snake_case(node)));

	quote! {
		fn validate_slot(parent: Self::Kind, index: usize, value: Option<Self::Kind>) -> bool {
			match parent {
				#(#unknown_kinds)|* | ERROR => true,
				#(#normal_node_arms),*,
				#(#lists),*,
				_ => unreachable!("Is {:?} a token?", parent),
			}
		}
	}
}

fn generate_validate_end(ast: &AstSrc) -> TokenStream {
	let normal_node_arms = ast.nodes.iter().map(|node| {
		let kind = format_ident!("{}", to_upper_snake_case(&node.name));
		let expected_len = node.fields.len();

		quote! {
			#kind => actual_len == #expected_len
		}
	});

	let lists = ast.lists().map(|(name, data)| {
		let kind = format_ident!("{}", to_upper_snake_case(name));
		let body = if let Some(separator) = &data.separator {
			let allow_trailing = separator.allow_trailing;

			if allow_trailing {
				quote! { true }
			} else {
				quote! {
					actual_len == 0 || actual_len % 2 == 1
				}
			}
		} else {
			quote! {
				true
			}
		};

		quote! { #kind => #body }
	});

	let unknown_kinds = ast
		.unknowns
		.iter()
		.map(|node| format_ident!("{}", to_upper_snake_case(node)));

	quote! {
		fn validate_end(parent: Self::Kind, actual_len: usize) -> bool {
			match parent {
				#(#unknown_kinds)|* | ERROR => true,
				#(#normal_node_arms),*,
				#(#lists),*,
				_ => unreachable!("Is {:?} a token?", parent),
			}
		}
	}
}
