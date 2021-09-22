use anyhow::Result;
use indexmap::IndexMap;
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use xshell::cmd;

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

#[derive(Deserialize, Serialize, Debug)]
pub struct ChildType {
	pub required: bool,
	pub multiple: bool,
	pub types: Vec<Child>,
}

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Grammar {
	pub name: String,
	pub nodes: Vec<NodeType>,
	pub token_kinds: Vec<String>,
	pub supertypes: Vec<String>,
}

impl Grammar {
	pub fn from_node_types(json: &str, name: &str) -> Result<Grammar> {
		let name = name.to_owned();
		let nodes: Vec<NodeType> = serde_json::from_str(json)?;

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

		Ok(Grammar {
			name,
			nodes,
			token_kinds,
			supertypes,
		})
	}
}

impl Grammar {
	pub fn create_syntax_kinds(&self) -> Result<String> {
		let (kinds, match_arms): (Vec<_>, Vec<_>) = self
			.nodes
			.iter()
			.map(|node| {
				let original_name = &node.kind;
				let new_name = format_ident!("{}", node.kind_name());
				let named = node.named;
				let arm = quote! {
					(#original_name, #named) => #new_name
				};
				(new_name, arm)
			})
			.unzip();

		let lang = format_ident!("{}", self.name.to_ascii_uppercase());
		let name = self.name.to_string();

		let tokenstream = quote! {
			#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
			#[repr(u16)]
			pub enum SyntaxKind {
			#(#kinds),*,
			Whitespace,
			ERROR,
			__LAST,
			}
			use SyntaxKind::*;
			use crate::ParserLanguage;

			pub struct Language {
				pub name: &'static str,
			}
			pub const #lang: Language = Language { name: #name };

			impl Language {
				pub fn get_syntax_kind(&self, name: &str, named: bool) -> SyntaxKind {
					match (name, named) {
						#(#match_arms),*,
						("ERROR", true) => SyntaxKind::ERROR,
						_ => panic!("Syntax Kind not found: {} / {}", name, named)
					}
				}
			}

			impl ParserLanguage for Language {
				type SyntaxNode = SyntaxNode;

				fn name(&self) -> &'static str {
					self.name
				}
				fn get_kind(&self, kind: &str, named: bool) -> u16 {
					self.get_syntax_kind(kind, named).into()
				}

				fn whitespace_kind(&self) -> u16 {
					SyntaxKind::Whitespace.into()
				}
				fn new_root(&self, green: rowan::GreenNode) -> Self::SyntaxNode {
					SyntaxNode::new_root(green)
				}
			}

			impl From<u16> for SyntaxKind {
				#[inline]
				fn from(d: u16) -> SyntaxKind {
					assert!(d <= (SyntaxKind::__LAST as u16));
					unsafe { std::mem::transmute::<u16, SyntaxKind>(d) }
				}
			}

			impl From<SyntaxKind> for u16 {
				#[inline]
				fn from(k: SyntaxKind) -> u16 {
					k as u16
				}
			}


			#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
			pub enum #lang {}
			impl rowan::Language for #lang {
				type Kind = SyntaxKind;

				fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
					SyntaxKind::from(raw.0)
				}

				fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
					rowan::SyntaxKind(kind.into())
				}
			}

			pub type SyntaxNode = rowan::SyntaxNode<#lang>;
			pub type SyntaxToken = rowan::SyntaxToken<#lang>;
			pub type SyntaxElement = rowan::SyntaxElement<#lang>;
			pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<#lang>;
			pub type SyntaxElementChildren = rowan::SyntaxElementChildren<#lang>;




		};

		let code = reformat_code(&tokenstream.to_string())?;
		Ok(code)
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
fn to_pascal_case(s: &str) -> String {
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
fn reformat_code(text: &str) -> Result<String> {
	let mut stdout = cmd!("rustfmt").stdin(text).read()?;
	if !stdout.ends_with('\n') {
		stdout.push('\n');
	}
	Ok(stdout)
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
