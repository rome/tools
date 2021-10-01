//#![allow(unused)]
pub mod ast_src;
pub mod sourcegen_ast;
pub mod syntax_kind;

use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use ast_src::{AstEnumSrc, AstNodeSrc, AstSrc, Cardinality};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use tree_sitter::Language;
use xshell::cmd;

use crate::{
	ast_src::Field,
	sourcegen_ast::{generate_nodes, generate_tokens},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct NodeType {
	pub named: bool,
	#[serde(rename = "type")]
	pub kind: String,
	// IndexMap is used to preserve order for iteration
	pub fields: Option<IndexMap<String, ChildType>>,
	pub children: Option<ChildType>,
	pub subtypes: Option<Vec<Child>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChildType {
	pub required: bool,
	pub multiple: bool,
	pub types: Vec<Child>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Child {
	pub named: bool,
	#[serde(rename = "type")]
	pub kind: String,
}

impl NodeType {
	pub fn kind_name(&self) -> String {
		make_kind_name(&self.kind, self.named)
	}

	pub fn is_supertype(&self) -> bool {
		self.subtypes.is_some()
	}

	pub fn is_token_type(&self) -> bool {
		self.fields.is_none() && self.children.is_none() && !self.is_supertype()
	}
}

impl Child {
	pub fn kind_name(&self) -> String {
		make_kind_name(&self.kind, self.named)
	}
}

#[derive(Debug)]
pub struct Grammar {
	pub name: String,
	pub language: Language,
	pub nodes: Vec<NodeType>,
	pub token_kinds: Vec<String>,
	pub supertypes: Vec<String>,
}

impl Grammar {
	pub fn new(node_types: &str, name: &str, language: Language) -> Result<Grammar> {
		let name = name.to_owned();
		let mut nodes: Vec<NodeType> = serde_json::from_str(node_types)?;

		let token_kinds = nodes
			.iter()
			.filter(|n| n.is_token_type())
			.map(|n| n.kind_name())
			.collect();

		let supertypes = nodes
			.iter()
			.filter(|n| n.is_supertype())
			.map(|n| n.kind_name())
			.collect();

		for node in &mut nodes {
			if let Some(children) = &node.children {
				let map = node.fields.get_or_insert(Default::default());
				map.insert(String::from("inner_children"), children.to_owned());
			}
		}

		Ok(Grammar {
			name,
			language,
			nodes,
			token_kinds,
			supertypes,
		})
	}
}

pub fn generate_nodes_from_grammars(grammars: &[Grammar]) -> String {
	let (ast_src, _) = create_ast_src(grammars);
	let result = generate_nodes(&ast_src);
	result
}

pub fn generate_tokens_from_grammars(grammars: &[Grammar]) -> String {
	let (ast_src, _) = create_ast_src(grammars);
	let result = generate_tokens(&ast_src);
	result
}

pub(crate) fn create_ast_src(grammars: &[Grammar]) -> (AstSrc, Vec<String>) {
	let mut ast_src = AstSrc::default();
	let conflicts = Vec::new();
	add_tokens(&mut ast_src, grammars);
	add_nodes_and_enums(&mut ast_src, grammars);

	(ast_src, conflicts)
}

fn add_tokens(ast_src: &mut AstSrc, grammars: &[Grammar]) {
	let mut tokens: BTreeSet<String> = BTreeSet::new();

	for grammar in grammars {
		for node in &grammar.nodes {
			if node.is_token_type() {
				tokens.insert(node.kind_name());
			}
		}
	}
	ast_src.tokens = tokens.into_iter().collect();
}

pub(crate) fn add_nodes_and_enums(ast_src: &mut AstSrc, grammars: &[Grammar]) {
	let mut supertypes: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
	let mut all_node_fields: BTreeMap<String, BTreeMap<String, Field>> = BTreeMap::new();
	let mut all_nodes: BTreeSet<String> = BTreeSet::new();

	for grammar in grammars {
		for node in &grammar.nodes {
			if let Some(subtypes) = &node.subtypes {
				add_variants(&mut supertypes, node.kind_name(), &subtypes);
				continue;
			}

			// if node.is_token_type() {
			// 	continue;
			// }

			all_nodes.insert(node.kind_name());

			if let Some(fields) = &node.fields {
				let node_fields = all_node_fields.entry(node.kind_name()).or_default();
				for (field_name, child) in fields {
					let cardinality = match child.multiple {
						true => Cardinality::Many,
						false => Cardinality::Optional,
					};
					if child.types.len() > 1 {
						let enum_name =
							format!("{}{}", node.kind_name(), to_pascal_case(field_name));
						add_variants(&mut supertypes, enum_name.clone(), &child.types);

						// TODO: Check for conflicts
						node_fields.insert(
							field_name.clone(),
							Field::Node {
								name: field_name.clone(),
								ty: enum_name,
								cardinality,
							},
						);
					} else if child.types.len() == 1 {
						let kind = &child.types[0];
						let ty = kind.kind_name();
						let field = match ast_src.tokens.contains(&ty) {
							true => Field::NamedToken {
								name: field_name.clone(),
								ty,
								cardinality,
							},
							false => Field::Node {
								name: field_name.clone(),
								ty,
								cardinality,
							},
						};
						// TODO: Check for conflicts
						node_fields.insert(field_name.clone(), field);
					}
				}
			}
		}
	}
	for (name, variants) in supertypes {
		let mut ast_enum_src = AstEnumSrc::default();
		ast_enum_src.name = name;
		ast_enum_src.variants = variants.into_iter().collect();

		ast_src.enums.push(ast_enum_src)
	}

	for name in all_nodes {
		let mut node = AstNodeSrc::default();
		node.name = name;
		if let Some(node_fields) = all_node_fields.remove(&node.name) {
			node.fields = node_fields.into_values().collect();
		}
		ast_src.nodes.push(node);
	}
}

fn add_variants(
	supertypes: &mut BTreeMap<String, BTreeSet<String>>,
	enum_name: String,
	children: &[Child],
) {
	let set = supertypes.entry(enum_name).or_default();
	for child in children {
		set.insert(child.kind_name());
	}
}

fn make_kind_name(ident: &str, named: bool) -> String {
	let name = sanitize_identifier(ident);
	let mut name = to_pascal_case(&name);
	let suffix = if named { "" } else { "Token" };
	name.push_str(suffix);

	// Manual replacements
	match name.as_str() {
		"String" => "StringLiteral".into(),
		_ => name,
	}
}

// From rust-analyzer
pub fn to_pascal_case(s: &str) -> String {
	let mut buf = String::with_capacity(s.len());
	let mut prev_is_underscore = true;
	for c in s.chars() {
		if c == '_' {
			prev_is_underscore = true;
		} else if prev_is_underscore {
			buf.push(c.to_ascii_uppercase());
			prev_is_underscore = false;
		} else {
			buf.push(c.to_ascii_lowercase());
		}
	}
	buf
}

pub fn to_upper_snake_case(s: &str) -> String {
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

pub fn reformat(text: &str) -> Result<String> {
	let mut stdout = cmd!("rustfmt").stdin(text).read()?;
	if !stdout.ends_with('\n') {
		stdout.push('\n');
	}
	Ok(stdout)
}

pub fn add_preamble(generator: &'static str, mut text: String) -> String {
	let preamble = format!("//! Generated by `{}`, do not edit by hand.\n\n", generator);
	text.insert_str(0, &preamble);
	text
}

// From https://github.com/tree-sitter/tree-sitter/blob/master/cli/src/generate/render.rs
fn sanitize_identifier(name: &str) -> String {
	let mut result = String::with_capacity(name.len());
	for c in name.chars() {
		if ('a'..='z').contains(&c)
			|| ('A'..='Z').contains(&c)
			|| ('0'..='9').contains(&c)
			|| c == '_'
		{
			result.push(c);
		} else {
			let replacement = match c {
				'~' => "TILDE",
				'`' => "BQUOTE",
				'!' => "BANG",
				'@' => "AT",
				'#' => "POUND",
				'$' => "DOLLAR",
				'%' => "PERCENT",
				'^' => "CARET",
				'&' => "AMP",
				'*' => "STAR",
				'(' => "LPAREN",
				')' => "RPAREN",
				'-' => "DASH",
				'+' => "PLUS",
				'=' => "EQ",
				'{' => "LBRACE",
				'}' => "RBRACE",
				'[' => "LBRACK",
				']' => "RBRACK",
				'\\' => "BSLASH",
				'|' => "PIPE",
				':' => "COLON",
				';' => "SEMI",
				'"' => "DQUOTE",
				'\'' => "SQUOTE",
				'<' => "LT",
				'>' => "GT",
				',' => "COMMA",
				'.' => "DOT",
				'?' => "QMARK",
				'/' => "SLASH",
				'\n' => "LF",
				'\r' => "CR",
				'\t' => "TAB",
				_ => continue,
			};
			if !result.is_empty() && !result.ends_with('_') {
				result.push('_');
			}
			result += replacement;
		}
	}
	result
}
