//! Much of this code is copied from [rust-analyzer] with some modifications
//!
//! This code is currently not being used here to its full potential. Also,
//! the implementation here is currently buggy through no fault of rust-analyzer.
//!
//! [rust-analyzer]: <https://github.com/rust-analyzer/rust-analyzer/blob/master/crates/syntax/src/tests/sourcegen_ast.rs>

use std::fmt::Write;

use quote::{format_ident, quote};

use crate::{
	ast_src::{AstSrc, Cardinality, Field},
	to_upper_snake_case,
};

pub(crate) fn generate_nodes(grammar: &AstSrc) -> String {
	let (node_defs, node_boilerplate_impls): (Vec<_>, Vec<_>) = grammar
		.nodes
		.iter()
		.map(|node| {
			let name = format_ident!("{}", node.name);
			let kind = format_ident!("{}", to_upper_snake_case(&node.name));
			let traits = node.traits.iter().map(|trait_name| {
				let trait_name = format_ident!("{}", trait_name);
				quote!(impl ast::#trait_name for #name {})
			});

			let methods = node.fields.iter().map(|field| {
				let method_name = field.method_name();
				let ty = field.ty();

				if field.is_many() {
					quote! {
						pub fn #method_name(&self) -> AstChildren<#ty> {
							support::children(&self.syntax)
						}
					}
				} else if let Some(token_kind) = field.token_kind() {
					quote! {
						pub fn #method_name(&self) -> Option<#ty> {
							support::token(&self.syntax, #token_kind)
						}
					}
				} else {
					quote! {
						pub fn #method_name(&self) -> Option<#ty> {
							support::child(&self.syntax)
						}
					}
				}
			});
			(
				quote! {
					#[pretty_doc_comment_placeholder_workaround]
					#[derive(Debug, Clone, PartialEq, Eq, Hash)]
					pub struct #name {
						pub(crate) syntax: SyntaxNode,
					}

					#(#traits)*

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

	let (enum_defs, enum_boilerplate_impls): (Vec<_>, Vec<_>) = grammar
		.enums
		.iter()
		.map(|en| {
			let variants: Vec<_> = en
				.variants
				.iter()
				.map(|var| format_ident!("{}", var))
				.collect();
			let name = format_ident!("{}", en.name);
			// let kinds: Vec<_> = variants
			// 	.iter()
			// 	.map(|name| format_ident!("{}", name.to_string()))
			// 	// .map(|name| format_ident!("{}", to_upper_snake_case(&name.to_string())))
			// 	.collect();
			let traits = en.traits.iter().map(|trait_name| {
				let trait_name = format_ident!("{}", trait_name);
				quote!(impl ast::#trait_name for #name {})
			});

			let (enums, kinds): (Vec<_>, Vec<_>) = variants
				.iter()
				.partition(|v| grammar.enums.iter().any(|e| e.name == v.to_string()));

			let syntax_kinds: Vec<_> = kinds
				.iter()
				.map(|k| format_ident!("{}", to_upper_snake_case(&k.to_string())))
				.collect();

			let can_cast_matchall = if enums.len() > 0 {
				quote! { _ => #(#enums::can_cast(kind))|* }
			} else {
				quote! { _ => false }
			};

			let ast_node = quote! {
				impl AstNode for #name {
					fn can_cast(kind: SyntaxKind) -> bool {
						match kind {
							#(SyntaxKind::#syntax_kinds)|* => true,
							#can_cast_matchall

						}
					}
					fn cast(syntax: SyntaxNode) -> Option<Self> {
						let res = match syntax.kind() {
							#(
							SyntaxKind::#syntax_kinds => #name::#kinds(#kinds { syntax }),
							)*
							#(
							kind if #enums::can_cast(kind) => #name::#enums(#enums::cast(syntax)?),
							)*
							_ => return None,
						};
						Some(res)
					}
					fn syntax(&self) -> &SyntaxNode {
						match self {
							#(
							#name::#kinds(it) => &it.syntax,
							)*
							#(
							#name::#enums(it) => &it.syntax(),
							)*
						}
					}
				}
			};

			(
				quote! {
					#[pretty_doc_comment_placeholder_workaround]
					#[derive(Debug, Clone, PartialEq, Eq, Hash)]
					pub enum #name {
						#(#variants(#variants),)*
					}

					#(#traits)*
				},
				quote! {
					#(
						impl From<#variants> for #name {
							fn from(node: #variants) -> #name {
								#name::#variants(node)
							}
						}
					)*
					#ast_node
				},
			)
		})
		.unzip();

	let enum_names = grammar.enums.iter().map(|it| &it.name);
	let node_names = grammar.nodes.iter().map(|it| &it.name);

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
			SyntaxNode, SyntaxToken, SyntaxKind::{self, *},
			ast::{AstNode, AstChildren, support},
		};

		#(#node_defs)*
		#(#enum_defs)*
		#(#node_boilerplate_impls)*
		#(#enum_boilerplate_impls)*
		#(#display_impls)*
	};

	let ast = ast.to_string();

	let mut res = String::with_capacity(ast.len() * 2);

	let mut docs = grammar
		.nodes
		.iter()
		.map(|it| &it.doc)
		.chain(grammar.enums.iter().map(|it| &it.doc));

	for chunk in ast.split("# [pretty_doc_comment_placeholder_workaround] ") {
		res.push_str(chunk);
		if let Some(doc) = docs.next() {
			write_doc_comment(doc, &mut res);
		}
	}

	let res = crate::reformat(&res).expect("Failed to reformat nodes.");
	crate::add_preamble("syntax_codegen", res)
}

pub(crate) fn generate_tokens(grammar: &AstSrc) -> String {
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

	let tokenstream = quote! {
		use crate::{SyntaxKind::{self, *}, SyntaxToken, ast::AstToken};
		#(#tokens)*
	};

	let res = tokenstream.to_string();
	let res = crate::reformat(&res).expect("Failed to reformat tokens.");
	crate::add_preamble("syntax_codegen", res)
}

fn write_doc_comment(contents: &[String], dest: &mut String) {
	for line in contents {
		writeln!(dest, "///{}", line).unwrap();
	}
}

impl Field {
	fn is_many(&self) -> bool {
		matches!(
			self,
			Field::Node {
				cardinality: Cardinality::Many,
				..
			}
		)
	}
	fn token_kind(&self) -> Option<proc_macro2::TokenStream> {
		match self {
			Field::Token(token) => {
				let token: proc_macro2::TokenStream = token.parse().unwrap();
				Some(quote! { SyntaxKind::#token })
			}
			Field::NamedToken { ty, .. } => {
				let token = format_ident!("{}", to_upper_snake_case(ty));
				Some(quote! { SyntaxKind::#token})
			}
			_ => None,
		}
	}
	fn method_name(&self) -> proc_macro2::Ident {
		match self {
			Field::Token(name) => {
				format_ident!("{}_token", name)
			}
			Field::Node { name, .. } | Field::NamedToken { name, .. } => {
				if name == "type" {
					format_ident!("ty")
				} else {
					format_ident!("{}", name)
				}
			}
		}
	}
	fn ty(&self) -> proc_macro2::Ident {
		match self {
			Field::Token(_) | Field::NamedToken { .. } => format_ident!("SyntaxToken"),
			// Field::NamedToken { ty, .. } => format_ident!("{}", ty),
			Field::Node { ty, .. } => format_ident!("{}", ty),
		}
	}
}
