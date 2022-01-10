use anyhow::Result;
use rslint_parser::{
	ast::{self, JsAnyExpression},
	AstNode,
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

			let message = format!("rome: do not use the {} operator", op.text_trimmed());
			let signal = Signal::diagnostic(op, message);
			Some(signal)
		})
		.collect()
}

fn is_null_literal(res: SyntaxResult<JsAnyExpression>) -> bool {
	match res {
		Ok(exp) => exp.syntax().kind() == JS_NULL_LITERAL_EXPRESSION,
		Err(_) => false,
	}
}
