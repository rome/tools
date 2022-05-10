use rome_js_syntax::{JsAnyRoot, TextRange};
use rome_rowan::AstNode;

mod analyzers;
mod assists;
mod categories;
mod registry;
mod signals;

pub use crate::categories::ActionCategory;
use crate::registry::RuleRegistry;
pub use crate::signals::{AnalyzerAction, AnalyzerDiagnostic, AnalyzerSignal};

/// Allows filtering the list of rules that will be executed in a run of the analyzer,
/// and at what source code range signals (diagnostics or actions) may be raised
#[derive(Default)]
pub struct AnalysisFilter<'a> {
    /// Only allow rules with these names to emit diagnostics
    pub rules: Option<&'a [&'a str]>,
    /// Only emit diagnostics matching this text range
    pub range: Option<TextRange>,
}

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call the `callback` when an analysis rule emits a diagnostic or action
pub fn analyze<B>(root: &JsAnyRoot, filter: AnalysisFilter, mut callback: B)
where
    B: FnMut(&dyn AnalyzerSignal),
{
    let registry = filter
        .rules
        .map(RuleRegistry::with_rules)
        .unwrap_or_default();

    for node in root.syntax().descendants() {
        if let Some(range) = filter.range {
            if node.text_range().ordering(range).is_ne() {
                continue;
            }
        }

        registry.analyze(root, node, &mut callback);
    }
}
