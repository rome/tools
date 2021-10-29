//! Generate SyntaxKind definitions as well as typed AST definitions for nodes and tokens.
//! This is derived from rust-analyzer/xtask/codegen

use super::{
	kinds_src::{AstSrc, Field},
	to_lower_snake_case, Mode,
};
use crate::{
	codegen::{
		self,
		generate_nodes::generate_nodes,
		generate_syntax_kinds::generate_syntax_kinds,
		generate_tokens::generate_tokens,
		kinds_src::{AstEnumSrc, AstNodeSrc, KINDS_SRC},
		update,
	},
	project_root, Result,
};
use ungrammar::{Grammar, Rule};

pub fn generate_ast(mode: Mode) -> Result<()> {
	let grammar_src = include_str!("../../js.ungram");
	let grammar: Grammar = grammar_src.parse().unwrap();
	let ast = make_ast(&grammar);

	let tokens_file = project_root().join(codegen::AST_TOKENS);
	let contents = generate_tokens(&ast)?;
	update(tokens_file.as_path(), &contents, mode)?;

	let ast_nodes_file = project_root().join(codegen::AST_NODES);
	let contents = generate_nodes(&ast)?;
	update(ast_nodes_file.as_path(), &contents, mode)?;

	let syntax_kinds_file = project_root().join(codegen::SYNTAX_KINDS);
	let contents = generate_syntax_kinds(KINDS_SRC)?;
	update(syntax_kinds_file.as_path(), &contents, mode)?;

	Ok(())
}

fn make_ast(grammar: &Grammar) -> AstSrc {
	let tokens = "String Number Whitespace Comment"
		.split_ascii_whitespace()
		.map(|it| it.to_string())
		.collect::<Vec<_>>();

	let mut ast = AstSrc {
		tokens,
		..Default::default()
	};
	for node in grammar.iter() {
		let name = grammar[node].name.clone();
		let rule = &grammar[node].rule;

		match handle_alternatives(grammar, rule) {
			Some(variants) => ast.enums.push(AstEnumSrc {
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
	has_many: bool,
) {
	match rule {
		Rule::Labeled { label, rule } => {
			// Some methods need to be manually implemented because they need some custom logic;
			// we use the prefix "manual__" to exclude labelled nodes.
			let manually_implemented = label.as_str().contains("manual__");

			if manually_implemented {
				return;
			}

			handle_rule(fields, grammar, rule, Some(label), optional, has_many)
		}
		Rule::Node(node) => {
			let ty = grammar[*node].name.clone();
			let name = label.cloned().unwrap_or_else(|| to_lower_snake_case(&ty));
			let field = Field::Node {
				name,
				ty,
				optional,
				has_many,
			};
			fields.push(field);
		}
		Rule::Token(token) => {
			let mut name = grammar[*token].name.clone();

			if name != "int_number" && name != "string" {
				// These tokens, when parsed to proc_macro2::TokenStream, generates a stream of bytes
				// that can't be recognized by [quote].
				// Hence, they need to be decorated with single quotes.
				if "[]{}()`".contains(&name) {
					name = format!("'{}'", name);
				}
				let field = Field::Token(name);
				fields.push(field);
			}
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
}
