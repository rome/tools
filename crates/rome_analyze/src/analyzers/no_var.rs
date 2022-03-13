use crate::{signals::DiagnosticExt, Analysis, Analyzer, AnalyzerContext};
use rome_js_syntax::{AstNode, JsVariableDeclaration};

pub const ANALYZER: Analyzer = Analyzer {
    name: "noVar",
    action_categories: &[],
    analyze,
};

fn analyze(ctx: &AnalyzerContext) -> Option<Analysis> {
    ctx.query_nodes::<JsVariableDeclaration>()
        .filter(|n| n.is_var())
        .map(|n| ctx.error(n.range(), "Do not use var").into_signal())
        .collect()
}
