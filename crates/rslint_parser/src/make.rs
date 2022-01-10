//! Temporary hacks that create AST nodes/tokens to assist with analyzer development
//!
//! This entire module should be replaced (likely with the help of codegen).

use crate::JsSyntaxKind;

use crate::ast::*;
use crate::AstNode;
use crate::SyntaxToken;
use crate::SyntaxTreeBuilder;

fn ast_from_text<N: AstNode>(text: &str) -> N {
	let parse = crate::parse_text(text, 0);
	let node = match parse.tree().syntax().descendants().find_map(N::cast) {
		Some(it) => it,
		None => {
			panic!(
				"Failed to make ast node `{}` from text {}",
				std::any::type_name::<N>(),
				text
			)
		}
	};
	let node = node.clone_subtree();
	assert_eq!(node.syntax().text_range().start(), 0.into());
	node
}

fn token_from_text(kind: JsSyntaxKind, text: &str) -> SyntaxToken {
	let mut builder = SyntaxTreeBuilder::new();
	builder.start_node(JsSyntaxKind::JS_SCRIPT);
	builder.token(kind, text);
	builder.finish_node();
	let node = builder.finish();
	node.first_token().unwrap()
}

pub fn punct_token(kind: JsSyntaxKind) -> Option<SyntaxToken> {
	if !kind.is_punct() {
		return None;
	}
	let text = kind.to_string()?;
	Some(token_from_text(kind, text))
}

impl JsStatementList {
	pub fn wrap(statement: JsAnyStatement) -> Self {
		ast_from_text(&statement.text())
	}
}

impl JsWhileStatement {
	pub fn quick(test: JsAnyExpression, body: JsAnyStatement) -> Self {
		let text = format!("while ({}) {}", test.text(), body.text());
		ast_from_text(&text)
	}
}

impl Default for JsEmptyStatement {
	fn default() -> Self {
		ast_from_text(";")
	}
}

impl JsBooleanLiteralExpression {
	pub fn quick(value: bool) -> Self {
		match value {
			true => ast_from_text("true"),
			false => ast_from_text("false"),
		}
	}
}

impl JsBlockStatement {
	/// Especially hacky implementation that (sort of) handles only tab indentation
	pub fn quick(statements: JsStatementList) -> Self {
		let mut text = String::from("{");
		let mut indent = 0;
		for node in statements
			.syntax_list()
			.iter()
			.filter_map(|s| s.into_node())
		{
			let leading = node
				.first_leading_trivia()
				.and_then(|t| t.pieces().last())
				.and_then(|p| p.as_whitespace());

			if let Some(whitespace) = leading {
				indent = whitespace.text().chars().filter(|c| c == &'\t').count()
			}
			text.push_str(&node.text().to_string())
		}
		text.push('\n');
		if indent > 1 {
			text.push_str(&"\t".repeat(indent - 1));
		}
		text.push('}');

		ast_from_text(&text)
	}
}

impl JsCaseClause {
	pub fn quick(test: JsAnyExpression, consequent: JsStatementList) -> Self {
		let text = format!(
			"switch true {{ case {}: {} }}",
			test.text(),
			consequent.text()
		);
		ast_from_text(&text)
	}
}
