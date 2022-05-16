use rome_diagnostics::file::FileId;
use rome_js_syntax::{
    suppression::{has_suppressions_category, SuppressionCategory},
    JsAnyRoot, TextRange, WalkEvent,
};
use rome_rowan::AstNode;

mod analyzers;
mod assists;
mod categories;
mod registry;
mod signals;

pub use crate::categories::{ActionCategory, RuleCategories, RuleCategory};
use crate::registry::RuleRegistry;
pub use crate::signals::{AnalyzerAction, AnalyzerSignal};

/// Allows filtering the list of rules that will be executed in a run of the analyzer,
/// and at what source code range signals (diagnostics or actions) may be raised
#[derive(Default)]
pub struct AnalysisFilter<'a> {
    /// Only allow rules with these categories to emit signals
    pub categories: RuleCategories,
    /// Only allow rules with these names to emit signals
    pub rules: Option<&'a [&'a str]>,
    /// Only emit signals matching this text range
    pub range: Option<TextRange>,
}

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call the `callback` when an analysis rule emits a diagnostic or action
pub fn analyze<B>(file_id: FileId, root: &JsAnyRoot, filter: AnalysisFilter, mut callback: B)
where
    B: FnMut(&dyn AnalyzerSignal),
{
    let registry = RuleRegistry::with_filter(&filter);

    let mut iter = root.syntax().preorder();
    while let Some(event) = iter.next() {
        let node = match event {
            WalkEvent::Enter(node) => node,
            WalkEvent::Leave(_) => continue,
        };

        if let Some(range) = filter.range {
            if node.text_range().ordering(range).is_ne() {
                iter.skip_subtree();
                continue;
            }
        }

        if has_suppressions_category(SuppressionCategory::Lint, &node) {
            iter.skip_subtree();
            continue;
        }

        registry.analyze(file_id, root, node, &mut callback);
    }
}

#[cfg(test)]
mod tests {
    use rome_js_parser::{parse, SourceType};

    use crate::{analyze, AnalysisFilter};

    #[test]
    fn suppression() {
        const SOURCE: &str = "
            // rome-ignore lint(noDoubleEquals): test
            function isEqual(a, b) {
                return a == b;
            }
        ";

        let parsed = parse(SOURCE, 0, SourceType::js_module());

        analyze(0, &parsed.tree(), AnalysisFilter::default(), |signal| {
            if let Some(diag) = signal.diagnostic() {
                assert_ne!(
                    diag.code,
                    Some(String::from("noDoubleEquals")),
                    "unexpected diagnostic signal raised"
                );
            }
        });
    }
}
