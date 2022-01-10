use anyhow::Result;
use rslint_parser::ast;

use crate::{ActionCategory, Analysis, Analyzer, AnalyzerContext, Signal};

pub fn create() -> Analyzer {
	Analyzer {
		name: "useWhile",
		action_categories: vec![ActionCategory::Suggestion],
		analyze,
	}
}

fn analyze(ctx: &AnalyzerContext) -> Result<Analysis> {
	ctx.query_nodes::<ast::JsForStatement>()
		.filter(|n| n.initializer().is_none() && n.update().is_none())
		.filter_map(|node| {
			let test = match node.test() {
				Some(test) => test,
				None => {
					let true_condition = ast::JsBooleanLiteralExpression::quick(true);
					ast::JsAnyExpression::JsAnyLiteralExpression(true_condition.into())
				}
			};

			let body = node.body().ok()?;
			let message = "rome: Use while loops instead of for loops";
			let action_title = "rome: Change to while loop";
			let replacement = ast::JsWhileStatement::quick(test, body);

			let signal = Signal::diagnostic_with_replacement(
				node,
				message,
				action_title,
				replacement,
				ActionCategory::SafeFix,
			);
			Some(signal)
		})
		.collect()
}
