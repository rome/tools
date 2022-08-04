use control_flow::make_visitor;
use rome_analyze::{
    AnalysisFilter, Analyzer, AnalyzerContext, AnalyzerSignal, ControlFlow, LanguageRoot, Phases,
    RegistryRuleMetadata, RuleAction, ServiceBag, SyntaxVisitor,
};
use rome_diagnostics::file::FileId;
use rome_js_syntax::{
    suppression::{parse_suppression_comment, SuppressionCategory},
    JsLanguage,
};

mod analyzers;
mod assists;
mod control_flow;
mod registry;
mod semantic_analyzers;
mod semantic_services;
pub mod utils;

use crate::{registry::build_registry, semantic_services::SemanticModelBuilderVisitor};

pub(crate) type JsRuleAction = RuleAction<JsLanguage>;

/// Return an iterator over the name and documentation of all the rules
/// implemented by the JS analyzer
pub fn metadata(filter: AnalysisFilter) -> impl Iterator<Item = RegistryRuleMetadata> {
    build_registry(&filter).metadata()
}

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action
pub fn analyze<'a, F, B>(
    file_id: FileId,
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    mut emit_signal: F,
) -> Option<B>
where
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    fn parse_linter_suppression_comment(text: &str) -> Vec<Option<&str>> {
        parse_suppression_comment(text)
            .flat_map(|comment| comment.categories)
            .filter_map(|(key, value)| {
                if key == SuppressionCategory::Lint {
                    Some(value)
                } else {
                    None
                }
            })
            .collect()
    }

    let mut analyzer = Analyzer::new(
        build_registry(&filter),
        parse_linter_suppression_comment,
        &mut emit_signal,
    );

    analyzer.add_visitor(Phases::Syntax, make_visitor());
    analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default());
    analyzer.add_visitor(Phases::Syntax, SemanticModelBuilderVisitor::new(root));

    analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default());

    analyzer.run(AnalyzerContext {
        file_id,
        root: root.clone(),
        range: filter.range,
        services: ServiceBag::default(),
    })
}

#[cfg(test)]
mod tests {

    use rome_analyze::Never;
    use rome_console::codespan::Severity;
    use rome_js_parser::parse;
    use rome_js_syntax::{SourceType, TextRange, TextSize};

    use crate::{analyze, AnalysisFilter, ControlFlow};

    #[test]
    fn suppression() {
        const SOURCE: &str = "
            function checkSuppressions1(a, b) {
                a == b;
                // rome-ignore lint(js): whole group
                a == b;
                // rome-ignore lint(js/noDoubleEquals): single rule
                a == b;
                /* rome-ignore lint(js/useWhile): multiple block comments */ /* rome-ignore lint(js/noDoubleEquals): multiple block comments */
                a == b;
                // rome-ignore lint(js/useWhile): multiple line comments
                // rome-ignore lint(js/noDoubleEquals): multiple line comments
                a == b;
                a == b;
            }

            // rome-ignore lint(js/noDoubleEquals): do not suppress warning for the whole function
            function checkSuppressions2(a, b) {
                a == b;
            }
        ";

        let parsed = parse(SOURCE, 0, SourceType::js_module());

        let mut error_ranges = Vec::new();
        analyze(0, &parsed.tree(), AnalysisFilter::default(), |signal| {
            if let Some(diag) = signal.diagnostic() {
                let diag = diag.into_diagnostic(Severity::Warning);
                let code = diag.code.as_deref().unwrap();
                let primary = diag.primary.as_ref().unwrap();

                if code == "js/noDoubleEquals" {
                    error_ranges.push(primary.span.range);
                }
            }

            ControlFlow::<Never>::Continue(())
        });

        assert_eq!(
            error_ranges.as_slice(),
            &[
                TextRange::new(TextSize::from(67), TextSize::from(69)),
                TextRange::new(TextSize::from(604), TextSize::from(606)),
                TextRange::new(TextSize::from(790), TextSize::from(792)),
            ]
        );
    }
}
