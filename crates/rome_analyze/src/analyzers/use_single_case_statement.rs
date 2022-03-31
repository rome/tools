use rome_js_syntax::JsCaseClause;
use rome_rowan::{AstNode, AstNodeList};

use crate::{signals::DiagnosticExt, Analysis, Analyzer, AnalyzerContext};

pub const ANALYZER: Analyzer = Analyzer {
    name: "useSingleCaseStatement",
    action_categories: &[],
    analyze,
};

fn analyze(ctx: &AnalyzerContext) -> Option<Analysis> {
    ctx.query_nodes::<JsCaseClause>()
        .filter(|n| n.consequent().len() > 1)
        .map(|node| {
            let message = "A switch case should only have a single statement.";
            ctx.error(node.range(), message).into_signal()
        })
        .collect()
}
