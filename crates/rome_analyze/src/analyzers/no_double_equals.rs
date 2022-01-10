use anyhow::Result;
use rslint_parser::{
	ast::{self, JsAnyExpression},
	make, AstNode,
	JsSyntaxKind::*,
	SyntaxResult,
};

use crate::{ActionCategory, Analysis, Analyzer, AnalyzerContext, Signal};

pub fn create() -> Analyzer {
	Analyzer {
		name: "noDoubleEquals",
		action_categories: vec![ActionCategory::SafeFix],
		analyze,
	}
}

fn analyze(ctx: &AnalyzerContext) -> Result<Analysis> {
	ctx.query_nodes::<ast::JsBinaryExpression>()
		.filter_map(|n| {
			let op = n.operator().ok()?;

			if !matches!(op.kind(), EQ2 | NEQ) {
				return None;
			}

			// TODO: Implement SyntaxResult helpers to make this cleaner
			if is_null_literal(n.left()) || is_null_literal(n.right()) {
				return None;
			}

			Some(op)
		})
		.map(|op| {
			let replacement = match op.kind() {
				EQ2 => make::punct_token(EQ3).unwrap(),
				NEQ => make::punct_token(NEQ2).unwrap(),
				_ => unreachable!(),
			};
			let message = format!("rome: do not use the {} operator", op.text_trimmed());
			let action_title = format!("rome: change to {}", replacement.text());

			Signal::diagnostic_with_replacement(
				op,
				message,
				action_title,
				replacement,
				ActionCategory::SafeFix,
			)
		})
		.collect()
}

fn is_null_literal(res: SyntaxResult<JsAnyExpression>) -> bool {
	match res {
		Ok(exp) => exp.syntax().kind() == JS_NULL_LITERAL_EXPRESSION,
		Err(_) => false,
	}
}
