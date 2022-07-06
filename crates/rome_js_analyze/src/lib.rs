use control_flow::make_visitor;
use rome_analyze::{
    AnalysisFilter, Analyzer, AnalyzerContext, AnalyzerSignal, ControlFlow, LanguageRoot, Phases,
    RuleAction, ServiceBag, ServiceBagData, SyntaxVisitor,
};
use rome_diagnostics::file::FileId;
use rome_js_semantic::semantic_model;
use rome_js_syntax::{suppression::parse_suppression_comment, JsLanguage};

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
pub fn metadata(filter: AnalysisFilter) -> impl Iterator<Item = (&'static str, &'static str)> {
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
    let mut analyzer = Analyzer::new(
        build_registry(&filter),
        |text| {
            parse_suppression_comment(text)
                .flat_map(|comment| comment.categories)
                .collect()
        },
        &mut emit_signal,
    );

    analyzer.add_visitor(make_visitor());

    analyzer.add_visitor(SyntaxVisitor::default());

    let breaking_reason = analyzer.run(AnalyzerContext {
        phase: Phases::Syntax,
        file_id,
        root: root.clone(),
        range: filter.range,
        services: ServiceBag::default(),
    });

    if breaking_reason.is_some() {
        return breaking_reason;
    }

    // Semantic Phase
    let model = semantic_model(root);
    let mut services = ServiceBagData::default();
    services.insert_service(model);

    let mut analyzer = Analyzer::new(
        build_registry(&filter),
        |text| {
            parse_suppression_comment(text)
                .flat_map(|comment| comment.categories)
                .collect()
        },
        &mut emit_signal,
    );

    analyzer.add_visitor(SyntaxVisitor::default());

    analyzer.run(AnalyzerContext {
        phase: Phases::Semantic,
        file_id,
        root: root.clone(),
        range: filter.range,
        services: ServiceBag::new(services),
    })
}

#[cfg(test)]
mod tests {

    use rome_analyze::Never;
    use rome_js_parser::parse;
    use rome_js_syntax::{SourceType, TextRange, TextSize};

    use crate::{analyze, AnalysisFilter, ControlFlow};

    #[test]
    fn suppression() {
        const SOURCE: &str = "
            function checkSuppressions1(a, b) {
                a == b;
                // rome-ignore lint(noDoubleEquals): single expression
                a == b;
                a == b;
            }

            // rome-ignore lint(noDoubleEquals): do not suppress warning for the whole function
            function checkSuppressions2(a, b) {
                a == b;
            }
        ";

        let parsed = parse(SOURCE, 0, SourceType::js_module());

        let mut error_ranges = Vec::new();
        analyze(0, &parsed.tree(), AnalysisFilter::default(), |signal| {
            if let Some(diag) = signal.diagnostic() {
                let code = diag.code.as_deref().unwrap();
                let primary = diag.primary.as_ref().unwrap();

                if code == "noDoubleEquals" {
                    error_ranges.push(primary.span.range);
                }
            }

            ControlFlow::<Never>::Continue(())
        });

        assert_eq!(
            error_ranges.as_slice(),
            &[
                TextRange::new(TextSize::from(67), TextSize::from(69)),
                TextRange::new(TextSize::from(186), TextSize::from(188)),
                TextRange::new(TextSize::from(369), TextSize::from(371)),
            ]
        );
    }
}
