use std::collections::HashMap;

use super::kinds_src::AstSrc;
use crate::{
	codegen::{kinds_src::Field, to_lower_snake_case, to_upper_snake_case},
	Result,
};
use quote::{format_ident, quote};

// these node won't generate any code
const BUILT_IN_TYPE: &str = "SyntaxElement";

pub fn generate_nodes(ast: &AstSrc) -> Result<String> {
	let filtered_enums: Vec<_> = ast
		.enums
		.iter()
		.filter(|e| e.name.as_str() != BUILT_IN_TYPE)
		.collect();

	let filtered_nodes: Vec<_> = ast
		.nodes
		.iter()
		.filter(|e| e.name.as_str() != BUILT_IN_TYPE)
		.collect();

	let (node_defs, node_boilerplate_impls): (Vec<_>, Vec<_>) = filtered_nodes
		.iter()
		.filter_map(|node| {
			if node.name.as_str() == BUILT_IN_TYPE {
				return None;
			}

			let name = format_ident!("{}", node.name);
			let kind = format_ident!("{}", to_upper_snake_case(node.name.as_str()));
			let mut slot = 0usize;

			let methods = node.fields.iter().map(|field| match field {
				Field::Token {
					token_kinds: tokens,
					name,
					..
				} => {
					// TODO: make the mandatory/optional bit
					let method_name = field.method_name();
					let token_kind = field.token_kind();
					let is_optional = field.is_optional();

					if !tokens.is_empty() {
						let tokens = field.token_kinds().unwrap();
						let method_name = format_ident!("{}", name);
						quote! {
							pub fn #method_name(&self) -> Option<SyntaxToken> {
								support::find_token(&self.syntax, #tokens)
							}
						}
					} else if is_optional {
						quote! {
							pub fn #method_name(&self) -> Option<SyntaxToken> {
								support::as_optional_token(&self.syntax, #token_kind)
							}
						}
					} else {
						quote! {
							pub fn #method_name(&self) -> SyntaxResult<SyntaxToken> {
								support::as_mandatory_token(&self.syntax, #token_kind)
							}
						}
					}
				}
				Field::Node {
					name: _,
					ty,
					optional,
					has_many,
					separated,
				} => {
					let is_built_in_tpe = &ty.eq(BUILT_IN_TYPE);
					let ty = format_ident!("{}", &ty);

					let method_name = field.method_name();
					// this is when we encounter a node that has "Unknown" in its name
					// it will return tokens a and nodes regardless because there's an error
					// inside the code
					if *is_built_in_tpe {
						quote! {
							pub fn #method_name(&self) -> SyntaxElementChildren {
								support::elements(&self.syntax)
							}
						}
					} else if *optional {
						quote! {
							pub fn #method_name(&self) -> Option<#ty> {
								support::as_optional_node(&self.syntax)
							}
						}
					} else if *has_many {
						let field = if *separated {
							quote! {
								pub fn #method_name(&self) -> AstSeparatedList<#ty> {
									support::separated_list(&self.syntax, #slot)
								}
							}
						} else {
							quote! {
								pub fn #method_name(&self) -> AstNodeList<#ty> {
									support::node_list(&self.syntax, #slot)
								}
							}
						};

						slot += 1;
						field
					} else {
						quote! {
							pub fn #method_name(&self) -> SyntaxResult<#ty> {
								support::as_mandatory_node(&self.syntax)
							}
						}
					}
				}
			});
			Some((
				quote! {
					// TODO: review documentation
					// #[doc = #documentation]
					#[derive(Debug, Clone, PartialEq, Eq, Hash)]
					pub struct #name {
						pub(crate) syntax: SyntaxNode,
					}

					impl #name {
						#(#methods)*
					}
				},
				quote! {
					impl AstNode for #name {
						fn can_cast(kind: SyntaxKind) -> bool {
							kind == #kind
						}
						fn cast(syntax: SyntaxNode) -> Option<Self> {
							if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
						}
						fn syntax(&self) -> &SyntaxNode { &self.syntax }
					}
				},
			))
		})
		.unzip();

	// it maps enum name A and its corresponding variants
	let name_to_variants: HashMap<_, _> = ast
		.enums
		.iter()
		.map(|current_enum| (current_enum.name.clone(), current_enum.variants.clone()))
		.collect();

	let (enum_defs, enum_boilerplate_impls): (Vec<_>, Vec<_>) = filtered_enums
		.iter()
		.map(|en| {
			// here we collect all the variants because this will generate the enums
			// so we don't care about filtered variants
			let variants_for_enum: Vec<_> = en
				.variants
				.iter()
				.map(|en| {
					let variant_name = format_ident!("{}", en);

					quote! {
						#variant_name(#variant_name)
					}
				})
				.collect();

			// Here we make the partition
			//
			// Inside an enum, we can have variants that point to a "flat" type or to another enum;
			// we want to divide these variants as we will generate a different code based on these requirements
			let (variant_of_variants, simple_variants): (Vec<_>, Vec<_>) =
				en.variants.iter().partition(|current_enum| {
					if let Some(variants) = name_to_variants.get(*current_enum) {
						!variants.is_empty()
					} else {
						false
					}
				});

			let variants: Vec<_> = simple_variants
				.iter()
				.map(|var| format_ident!("{}", var))
				.collect();

			let name = format_ident!("{}", en.name);
			let kinds: Vec<_> = variants
				.iter()
				.map(|name| format_ident!("{}", to_upper_snake_case(&name.to_string())))
				.collect();

			let variant_cast: Vec<_> = simple_variants
				.iter()
				.map(|current_enum| {
					let variant_is_enum = filtered_enums.iter().find(|e| &e.name == *current_enum);
					let variant_name = format_ident!("{}", current_enum);

					if variant_is_enum.is_some() {
						quote! {
							#variant_name::cast(syntax)?
						}
					} else {
						quote! {
							#variant_name { syntax }
						}
					}
				})
				.collect();

			// variant of variants
			let vv: Vec<_> = variant_of_variants
				.iter()
				.enumerate()
				.map(|(i, en)| {
					let variant_name = format_ident!("{}", en);
					let variable_name = format_ident!("{}", to_lower_snake_case(en.as_str()));
					(
						// cast() code
						if i != variant_of_variants.len() - 1 {
							quote! {
							if let Some(#variable_name) = #variant_name::cast(syntax.clone()) {
									return Some(#name::#variant_name(#variable_name));
							}}
						} else {
							// if this is the last variant, do not clone syntax
							quote! {
								if let Some(#variable_name) = #variant_name::cast(syntax) {
									return Some(#name::#variant_name(#variable_name));
							}}
						},
						// can_cast() code
						quote! {
							k if #variant_name::can_cast(k) => true,
						},
						// syntax() code
						quote! {
							#name::#variant_name(it) => it.syntax()
						},
					)
				})
				.collect();

			let vv_cast = vv.iter().map(|v| v.0.clone());

			let vv_can_cast = vv.iter().map(|v| v.1.clone());
			let vv_syntax = vv.iter().map(|v| v.2.clone());

			let all_kinds = if !kinds.is_empty() {
				quote! {
					#(#kinds)|* => true,
				}
			} else {
				quote! {}
			};

			let cast_fn = if !kinds.is_empty() {
				quote! {
					let res = match syntax.kind() {
						#(
							#kinds => #name::#variants(#variant_cast),
						)*
						_ =>  {
							#(
								#vv_cast
							)*
							return None
						}
					};
					Some(res)
				}
			} else {
				quote! {
						#(
						#vv_cast
					)*
					None
				}
			};
			let variant_can_cast: Vec<_> = simple_variants
				.iter()
				.map(|_| {
					quote! {
						&it.syntax
					}
				})
				.collect();

			(
				quote! {
					// #[doc = #doc]
					#[derive(Debug, Clone, PartialEq, Eq, Hash)]
					pub enum #name {
						#(#variants_for_enum),*
					}
				},
				quote! {
					#(
					impl From<#variants> for #name {
						fn from(node: #variants) -> #name {
							#name::#variants(node)
						}
					}
					)*

					impl AstNode for #name {
						fn can_cast(kind: SyntaxKind) -> bool {
							// matches!(kind, #(#kinds)|*)
							match kind {
								#all_kinds
								#(#vv_can_cast)*
								_ => false
							}
						}
						fn cast(syntax: SyntaxNode) -> Option<Self> {
								#cast_fn
						}
						fn syntax(&self) -> &SyntaxNode {
							match self {
								#(
								#name::#variants(it) => #variant_can_cast,
								)*
								#(
									#vv_syntax
								),*

							}
						}
					}
				},
			)
		})
		.unzip();

	let enum_names = filtered_enums.iter().map(|it| &it.name);
	let node_names = filtered_nodes.iter().map(|it| &it.name);

	let display_impls = enum_names
		.chain(node_names.clone())
		.map(|it| format_ident!("{}", it))
		.map(|name| {
			quote! {
				impl std::fmt::Display for #name {
					fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
						std::fmt::Display::fmt(self.syntax(), f)
					}
				}
			}
		});

	let ast = quote! {
	#![allow(clippy::enum_variant_names)]
	// sometimes we generate comparison of simple tokens
	#![allow(clippy::match_like_matches_macro)]

	use crate::{
		ast::*,
		SyntaxKind::{self, *},
		SyntaxNode, SyntaxToken, T,
		SyntaxResult
	};


		#(#node_defs)*
		#(#enum_defs)*
		#(#node_boilerplate_impls)*
		#(#enum_boilerplate_impls)*
		#(#display_impls)*
	};

	let ast = ast
		.to_string()
		.replace("T ! [ ", "T![")
		.replace(" ] )", "])");

	let pretty = crate::reformat(ast)?;
	Ok(pretty)
}
