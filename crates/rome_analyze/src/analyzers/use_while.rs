use rslint_parser::{ast, AstNode};

use crate::{signals::DiagnosticExt, Analysis, Analyzer, AnalyzerContext};

pub fn create() -> Analyzer {
	Analyzer {
		name: "useWhile",
		action_categories: vec![],
		analyze,
	}
}

fn analyze(ctx: &AnalyzerContext) -> Option<Analysis> {
	ctx.query_nodes::<ast::JsForStatement>()
		.filter(|n| n.initializer().is_none() && n.update().is_none())
		.map(|node| {
			let message = "Use a while loop instead of a for loop";
			ctx.error(node.range(), message).into_signal()
		})
		.collect()
}
