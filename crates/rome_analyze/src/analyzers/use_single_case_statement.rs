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
		.map(|node| {
			let message = "rome: A switch case should only have a single statement.";
			Signal::diagnostic(node, message)
		})
		.collect()
}
