use rome_js_syntax::{AstNode, JsForStatement};

use crate::{signals::DiagnosticExt, Analysis, Analyzer, AnalyzerContext};

pub const ANALYZER: Analyzer = Analyzer {
    name: "useWhile",
    action_categories: &[],
    analyze,
};

fn analyze(ctx: &AnalyzerContext) -> Option<Analysis> {
    ctx.query_nodes::<JsForStatement>()
        .filter(|n| n.initializer().is_none() && n.update().is_none())
        .map(|node| {
            let message = "Use a while loop instead of a for loop";
            ctx.error(node.range(), message).into_signal()
        })
        .collect()
}
