use anyhow::Result;
use rslint_parser::{ast, JsSyntaxKind};

use crate::{Analysis, Analyzer, AnalyzerContext, Signal};

pub fn create() -> Analyzer {
	Analyzer {
		name: "noVar",
		action_categories: vec![],
		analyze,
	}
}

fn analyze(ctx: &AnalyzerContext) -> Result<Analysis> {
	ctx.query_nodes::<ast::JsVariableDeclarations>()
		.filter(|n| n.kind().map(|k| k.kind()).ok() == Some(JsSyntaxKind::VAR_KW))
		.map(|n| Signal::diagnostic(n, "rome: do not use var"))
		.collect()
}
