use super::kinds_src::AstSrc;
use crate::codegen::generate_nodes::token_kind_to_code;
use crate::codegen::kinds_src::TokenKind;
use crate::{
	codegen::{kinds_src::Field, to_lower_snake_case, to_upper_snake_case},
	Result,
};
use quote::{format_ident, quote};

pub fn generate_js_tree_shape(ast: &AstSrc) -> Result<String> {
	let normal_node_fns = ast.nodes.iter().map(|node| {
		let kind = format_ident!("{}", to_upper_snake_case(&node.name));
		let expected_len = node.fields.len();

		let fields = node.fields.iter().map(|field| {
			let field_predicate = match field {
				Field::Node { ty, .. } => {
					let ast_type_name = format_ident!("{}", ty);

					quote! {
						#ast_type_name::can_cast(element.kind())
					}
				}
				Field::Token { kind, .. } => match kind {
					TokenKind::Single(expected) => {
						let expected_kind = token_kind_to_code(expected);
						quote! { element.kind() == #expected_kind}
					}
					TokenKind::Many(expected) => {
						let expected_kinds = expected.iter().map(|kind| token_kind_to_code(kind));
						quote! {
							matches!(element.kind(), #(#expected_kinds)|*)
						}
					}
				},
			};

			quote! {
				if let Some(element) = &current_element {
					if #field_predicate {
						slots[current_slot_index]= current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
			}
		});

		let fn_name = format_ident!("make_{}", to_lower_snake_case(&node.name));
		quote! {
			#[allow(unused_mut)]
			#[allow(unused_assignments)]
			#[allow(unused_variables)]
			fn #fn_name(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
				let actual_len = children.len();
				if actual_len > #expected_len {
					return RawSyntaxNode::new(#kind.to_unknown(), children.into_iter().map(Some));
				}

				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; #expected_len] = Default::default();
				let mut current_element = elements.next();

				#(#fields)*

				// Additional unexpected elements
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						#kind.to_unknown(),
						UnknownNodeChildrenIterator::new(
							&mut slots,
							actual_len,
							element,
							elements,
						),
					);
				}

				RawSyntaxNode::new(#kind, slots)
			}
		}
	});

	let normal_node_arms = ast.nodes.iter().map(|node| {
		let kind = format_ident!("{}", to_upper_snake_case(&node.name));
		let fn_name = format_ident!("make_{}", to_lower_snake_case(&node.name));

		quote! {
			#kind => Self::#fn_name(children)
		}
	});

	let lists = ast.lists().map(|(name, data)| {
		let element_type = format_ident!("{}", data.element_name);
		let kind = format_ident!("{}", to_upper_snake_case(name));
		if let Some(separator) = &data.separator {
			let allow_trailing = separator.allow_trailing;
			let separator_kind = token_kind_to_code(&separator.separator_token);
			quote! {
				#kind => Self::make_separated_list_syntax(kind, children, #element_type::can_cast, #separator_kind, #allow_trailing)
			}
		} else {
			quote! {
				#kind => Self::make_node_list_syntax(kind, children, #element_type::can_cast)
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
			JsSyntaxKind::*
		};

		use rome_rowan::{ParsedChildren, SyntaxKind, SyntaxFactory, RawSyntaxElement, RawSyntaxNode, UnknownNodeChildrenIterator};

		#[derive(Debug)]
		pub struct JsSyntaxFactory;


		impl SyntaxFactory for JsSyntaxFactory {
			type Kind = JsSyntaxKind;

			fn make_syntax(
				kind: Self::Kind,
				children: ParsedChildren<Self::Kind>,
			) -> RawSyntaxNode<Self::Kind>
			{
				match kind {
					#(#unknown_kinds)|* | ERROR => {
						RawSyntaxNode::new(kind, children.into_iter().map(Some))
					},
					#(#normal_node_arms),*,
					#(#lists),*,
					_ => unreachable!("Is {:?} a token?", kind),
				}
			}
		}

		impl JsSyntaxFactory {
			#(#normal_node_fns)*
		}
	};

	let pretty = crate::reformat(output)?;
	Ok(pretty)
}
