use super::kinds_src::AstSrc;
use crate::{
	codegen::{kinds_src::Field, to_upper_snake_case},
	Result,
};
use quote::{format_ident, quote};

pub fn generate_nodes(ast: &AstSrc) -> Result<String> {
	let (node_defs, node_boilerplate_impls): (Vec<_>, Vec<_>) = ast
		.nodes
		.iter()
		.map(|node| {
			let name = format_ident!("{}", node.name);
			let kind = format_ident!("{}", to_upper_snake_case(node.name.as_str()));
			// let documentation = node.documentation;

			let methods = node.fields.iter().map(|field| match field {
				Field::Token(_) => {
					// let name = format_ident!("{}", to_lower_snake_case(node.name.as_str()));
					// TODO: make the mandatory/optional bit
					let method_name = field.method_name();
					let token_kind = field.token_kind();

					quote! {
						pub fn #method_name(&self) -> Option<SyntaxToken> {
							support::token(&self.syntax, #token_kind)
						}
					}
				}
				Field::Node {
					name: _,
					ty,
					optional,
					has_many,
				} => {
					let ty = format_ident!("{}", &ty);

					let method_name = field.method_name();
					if *optional {
						quote! {
							pub fn #method_name(&self) -> Option<#ty> {
								support::child(&self.syntax)
							}
						}
					} else if *has_many {
						quote! {
							pub fn #method_name(&self) -> AstChildren<#ty> {
								support::children(&self.syntax)
							}
						}
					} else {
						quote! {
							pub fn #method_name(&self) -> Option<#ty> {
								support::child(&self.syntax)
							}
						}
					}
				}
			});
			(
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
			)
		})
		.unzip();

	let (enum_defs, enum_boilerplate_impls): (Vec<_>, Vec<_>) = ast
		.enums
		.iter()
		.map(|en| {
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

			let variants: Vec<_> = en
				.variants
				.iter()
				.map(|var| format_ident!("{}", var))
				.collect();

			let name = format_ident!("{}", en.name);
			let kinds: Vec<_> = variants
				.iter()
				.map(|name| format_ident!("{}", to_upper_snake_case(&name.to_string())))
				.collect();
			// let doc = en.documentation;

			let variant_cast: Vec<_> = en
				.variants
				.iter()
				.map(|current_enum| {
					let variant_is_enum = ast.enums.iter().find(|e| &e.name == current_enum);
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

			let variant_can_cast: Vec<_> = en
				.variants
				.iter()
				.map(|current_enum| {
					let variant_is_enum = ast.enums.iter().find(|e| &e.name == current_enum);

					if variant_is_enum.is_some() {
						quote! {
							it.syntax()
						}
					} else {
						quote! {
							&it.syntax
						}
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
							matches!(kind, #(#kinds)|*)
						}
						fn cast(syntax: SyntaxNode) -> Option<Self> {
							let res = match syntax.kind() {
								#(
								#kinds => #name::#variants(#variant_cast),
								)*
								_ => return None,
							};
							Some(res)
						}
						fn syntax(&self) -> &SyntaxNode {
							match self {
								#(
								#name::#variants(it) => #variant_can_cast,
								)*
							}
						}
					}
				},
			)
		})
		.unzip();

	let enum_names = ast.enums.iter().map(|it| &it.name);
	let node_names = ast.nodes.iter().map(|it| &it.name);

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
	use crate::{
		ast::*,
		SyntaxKind::{self, *},
		SyntaxNode, SyntaxToken, T,
	};
			#[allow(clippy::enum_variant_names)]

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
