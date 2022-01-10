use anyhow::Result;
use rslint_parser::{ast, AstNodeList};

use crate::{ActionCategory, Analysis, Analyzer, AnalyzerContext, Signal};

pub fn create() -> Analyzer {
	Analyzer {
		name: "useSingleCaseStatement",
		action_categories: vec![ActionCategory::Suggestion],
		analyze,
	}
}

fn analyze(ctx: &AnalyzerContext) -> Result<Analysis> {
	ctx.query_nodes::<ast::JsCaseClause>()
		.filter(|n| n.consequent().len() > 1)
		.filter_map(|node| {
			let message = "rome: A switch case should only have a single statement.";
			let action_title = "rome: Wrap case body in block";

			let test = node.test().ok()?;
			let block = ast::JsBlockStatement::quick(node.consequent());
			let list = ast::JsStatementList::wrap(block.into());
			let replacement = ast::JsCaseClause::quick(test, list);

			let signal = Signal::diagnostic_with_replacement(
				node,
				message,
				action_title,
				replacement,
				ActionCategory::Suggestion,
			);
			Some(signal)
		})
		.collect()
}
