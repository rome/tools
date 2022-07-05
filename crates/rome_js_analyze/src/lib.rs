use rome_analyze::{
    AnalysisFilter, Analyzer, AnalyzerSignal, ControlFlow, LanguageRoot, Never, Phases, RuleAction,
    ServiceBag, ServiceBagData, SyntaxVisitor, VisitorContext,
};
use rome_diagnostics::file::FileId;
use rome_js_semantic::semantic_model;
use rome_js_syntax::{
    suppression::{has_suppressions_category, SuppressionCategory},
    JsLanguage,
};

mod analyzers;
mod assists;
mod control_flow;
mod registry;
mod semantic_analyzers;
mod semantic_services;

use crate::registry::build_registry;

pub(crate) type JsRuleAction = RuleAction<JsLanguage>;

/// Return an iterator over the name and documentation of all the rules
/// implemented by the JS analyzer
pub fn metadata(filter: AnalysisFilter) -> Vec<(&'static str, &'static str)> {
    fn dummy_signal(_: &dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<Never> {
        panic!()
    }

    build_registry(&filter, dummy_signal).metadata()
}

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call the `callback` when an analysis rule emits a diagnostic or action
pub fn analyze<F, B>(
    file_id: FileId,
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    callback: F,
) -> Option<B>
where
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B>,
    B: 'static,
{
    let mut registry = build_registry(&filter, callback);

    // Syntax Phase
    let services = ServiceBag::default();

    let mut analyzer = Analyzer::<JsLanguage, B>::empty();
    analyzer.add_visitor(control_flow::make_visitor());
    analyzer.add_visitor(SyntaxVisitor::new(|node| {
        has_suppressions_category(SuppressionCategory::Lint, node)
    }));
    let breaking_reason = analyzer.run(VisitorContext {
        file_id,
        root: root.clone(),
        range: filter.range,
        match_query: Box::new(|file_id, root, query_match| {
            registry.match_query(Phases::Syntax, file_id, root, query_match, &services)
        }),
    });

    if breaking_reason.is_some() {
        return breaking_reason;
    }

    // Semantic Phase
    let model = semantic_model(root);
    let mut services = ServiceBagData::default();
    services.insert_service(model);
    let services = ServiceBag::new(services);

    let mut analyzer = Analyzer::<JsLanguage, B>::empty();
    analyzer.add_visitor(SyntaxVisitor::new(|node| {
        has_suppressions_category(SuppressionCategory::Lint, node)
    }));
    analyzer.run(VisitorContext {
        file_id,
        root: root.clone(),
        range: filter.range,
        match_query: Box::new(|file_id, root, query_match| {
            registry.match_query(Phases::Semantic, file_id, root, query_match, &services)
        }),
    })
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
