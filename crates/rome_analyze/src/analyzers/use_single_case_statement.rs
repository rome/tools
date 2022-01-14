use rslint_parser::{ast, AstNode, AstNodeList};

use crate::{signals::DiagnosticExt, Analysis, Analyzer, AnalyzerContext};

pub fn create() -> Analyzer {
    Analyzer {
        name: "useSingleCaseStatement",
        action_categories: vec![],
        analyze,
    }
}

fn analyze(ctx: &AnalyzerContext) -> Option<Analysis> {
    ctx.query_nodes::<ast::JsCaseClause>()
        .filter(|n| n.consequent().len() > 1)
        .map(|node| {
            let message = "A switch case should only have a single statement.";
            ctx.error(node.range(), message).into_signal()
        })
        .collect()
}
