use rome_analyze::{
    AnalysisFilter, Analyzer, AnalyzerSignal, ControlFlow, LanguageRoot, MetadataIter, RuleAction,
};
use rome_diagnostics::file::FileId;
use rome_js_syntax::{
    suppression::{has_suppressions_category, SuppressionCategory},
    JsLanguage,
};

mod analyzers;
mod assists;
mod registry;

use crate::registry::build_registry;

pub(crate) type JsRuleAction = RuleAction<JsLanguage>;

/// Return an iterator over the name and documentation of all the rules
/// implemented by the JS analyzer
pub fn metadata() -> MetadataIter<JsLanguage> {
    build_registry(&AnalysisFilter::default()).metadata()
}

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call the `callback` when an analysis rule emits a diagnostic or action
pub fn analyze<B>(
    file_id: FileId,
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    callback: impl FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B>,
) -> Option<B> {
    let registry = build_registry(&filter);

    let analyzer = Analyzer::new(registry, |node| {
        has_suppressions_category(SuppressionCategory::Lint, node)
    });

    analyzer.analyze(file_id, root, filter.range, callback)
}

#[cfg(test)]
mod tests {

    use rome_analyze::Never;
    use rome_js_parser::parse;
    use rome_js_syntax::SourceType;

    use crate::{analyze, AnalysisFilter, ControlFlow};

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

            ControlFlow::<Never>::Continue(())
        });
    }
}
