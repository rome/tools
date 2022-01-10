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
		.map(|node| {
			let message = "rome: Use a while loop instead of a for loop";
			Signal::diagnostic(node, message)
		})
		.collect()
}
