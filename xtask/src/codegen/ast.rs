//! Generate SyntaxKind definitions as well as typed AST definitions for nodes and tokens.
//! This is derived from rust-analyzer/xtask/codegen

use crate::{
	ast_src::{AstEnumSrc, AstNodeSrc, AstSrc, Field, KindsSrc, KINDS_SRC},
	codegen::{self, update, Mode},
	project_root, Result,
};
use proc_macro2::{Punct, Spacing};
use quote::{format_ident, quote};
use ungrammar::{Grammar, Rule};

pub fn generate_ast(mode: Mode) -> Result<()> {
	let grammar_src = include_str!("../../js.ungram");
	let grammar: Grammar = grammar_src.parse().unwrap();
	let ast = make_ast(&grammar);

	let ast_nodes_file = project_root().join(codegen::AST_NODES);
	let contents = generate_nodes(KINDS_SRC, &ast)?;
	update(ast_nodes_file.as_path(), &contents, mode)?;

	// let ast_tokens_file = project_root().join(codegen::AST_TOKENS);
	// let contents = generate_tokens(KINDS_SRC, &ast)?;
	// update(ast_tokens_file.as_path(), &contents, mode)?;

	Ok(())
}

fn make_ast(grammar: &Grammar) -> AstSrc {
	let tokens = "String Number"
		.split_ascii_whitespace()
		.map(|it| it.to_string())
		.collect::<Vec<_>>();

	let mut ast = AstSrc {
		tokens,
		..Default::default()
	};
	let nodes: Vec<_> = grammar.iter().collect();
	for &node in &nodes {
		let name = grammar[node].name.clone();
		let rule = &grammar[node].rule;
		match handle_alternatives(grammar, rule) {
			Some(variants) => ast.enums.push(AstEnumSrc {
				// TODO: to fill
				documentation: vec![],
				name,
				variants,
			}),
			None => {
				let mut fields = vec![];
				handle_rule(&mut fields, grammar, rule, None, false, false);
				ast.nodes.push(AstNodeSrc {
					documentation: vec![],
					name,
					fields,
				})
			}
		}
	}
	ast
}

fn handle_alternatives(grammar: &Grammar, rule: &Rule) -> Option<Vec<String>> {
	match rule {
		// this is for enums
		Rule::Alt(alternatives) => {
			let mut all_alternatives = vec![];
			for alternative in alternatives {
				match alternative {
					Rule::Node(it) => all_alternatives.push(grammar[*it].name.clone()),
					Rule::Token(it) if grammar[*it].name == ";" => (),
					_ => return None,
				}
			}
			Some(all_alternatives)
		}
		_ => None,
	}
}

fn handle_rule(
	fields: &mut Vec<Field>,
	grammar: &Grammar,
	rule: &Rule,
	label: Option<&String>,
	optional: bool,
	has_many: bool
) -> () {
	match rule {
		Rule::Labeled { label, rule } => {
			// let ty = grammar[*node].name.clone();
			// let name = label.cloned().unwrap_or_else(|| to_lower_snake_case(&ty));

			// dbg!(&ty);
			// dbg!(&name);
			// let field = Field::Node {
			// 	name,
			// 	ty,
			// 	cardinality: Cardinality::Optional,
			// };
			// fields.push(field);
		}
		Rule::Node(node) => {
			let ty = grammar[*node].name.clone();
			let name = label.cloned().unwrap_or_else(|| to_lower_snake_case(&ty));

			let field = Field::Node { name, ty, optional, has_many };
			fields.push(field);
		}
		Rule::Token(token) => {
			let name = grammar[*token].name.clone();
			let token = Field::Token(name);
			fields.push(token);
		}

		Rule::Rep(rule) => {
			handle_rule(fields, grammar, rule, label, false, true);
		}
		Rule::Opt(rule) => {
			handle_rule(fields, grammar, rule, label, true, false);
		}
		Rule::Seq(rules) | Rule::Alt(rules) => {
			for rule in rules {
				handle_rule(fields, grammar, rule, label, false, false);
			}
		}
	};

	()
}

fn generate_tokens(grammar: AstSrc) -> Result<String> {
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
		use crate::{SyntaxKind::{self, *}, SyntaxToken, ast::AstToken};
		#(#tokens)*
	})?
	.replace("#[derive", "\n#[derive");
	Ok(pretty)
}

fn generate_nodes(kinds: KindsSrc, ast: &AstSrc) -> Result<String> {
	let (node_defs, node_boilerplate_impls): (Vec<_>, Vec<_>) = ast
		.nodes
		.iter()
		.map(|node| {
			let name = format_ident!("{}", node.name);
			let kind = format_ident!("{}", to_upper_snake_case(node.name.as_str()));
			// let documentation = node.documentation;

			let methods = node.fields.iter().map(|field| match field {
				Field::Token(token) => {
					// let name = format_ident!("{}", to_lower_snake_case(node.name.as_str()));
					// TODO: make the mandatory/optional bit
					let method_name = field.method_name();
					let is_many = field.is_many();
					quote! {
						pub fn #method_name(&self) -> Option<SyntaxToken> {
							support::token(&self.syntax)
						}
					}
				}
				Field::Node { name, ty, optional, has_many } => {
					let ty = format_ident!("{}", &ty);
					let method_name = field.method_name();
					if *optional {
						quote! {
							pub fn #method_name(&self) -> Option<#ty> {
								support::child(&self.syntax)
							}
						}
					} else if *has_many == true {
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

			(
				quote! {
					// #[doc = #doc]
					#[derive(Debug, Clone, PartialEq, Eq, Hash)]
					pub enum #name {
						#(#variants(#variants),)*
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
								#kinds => #name::#variants(#variants { syntax }),
								)*
								_ => return None,
							};
							Some(res)
						}
						fn syntax(&self) -> &SyntaxNode {
							match self {
								#(
								#name::#variants(it) => &it.syntax,
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
			SyntaxNode, SyntaxToken, SyntaxKind::{self, *},
			ast::*,
			T,
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

fn generate_syntax_kinds(grammar: KindsSrc) -> Result<String> {
	let (single_byte_tokens_values, single_byte_tokens): (Vec<_>, Vec<_>) = grammar
		.punct
		.iter()
		.filter(|(token, _name)| token.len() == 1)
		.map(|(token, name)| (token.chars().next().unwrap(), format_ident!("{}", name)))
		.unzip();

	let punctuation_values = grammar.punct.iter().map(|(token, _name)| {
		if "{}[]()".contains(token) {
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

fn to_upper_snake_case(s: &str) -> String {
	let mut buf = String::with_capacity(s.len());
	let mut prev = false;
	for c in s.chars() {
		if c.is_ascii_uppercase() && prev {
			buf.push('_')
		}
		prev = true;

		buf.push(c.to_ascii_uppercase());
	}
	buf
}

fn to_lower_snake_case(s: &str) -> String {
	let mut buf = String::with_capacity(s.len());
	let mut prev = false;
	for c in s.chars() {
		if c.is_ascii_uppercase() && prev {
			buf.push('_')
		}
		prev = true;

		buf.push(c.to_ascii_lowercase());
	}
	buf
}
