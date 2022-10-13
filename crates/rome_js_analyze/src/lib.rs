use control_flow::make_visitor;
use rome_analyze::{
    AnalysisFilter, Analyzer, AnalyzerContext, AnalyzerOptions, AnalyzerSignal, ControlFlow,
    InspectMatcher, LanguageRoot, MatchQueryParams, MetadataRegistry, Phases, RuleAction,
    RuleRegistry, ServiceBag, SyntaxVisitor,
};
use rome_diagnostics::file::FileId;
use rome_js_syntax::{
    suppression::{parse_suppression_comment, SuppressionCategory},
    JsLanguage,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, error::Error};

mod analyzers;
mod assists;
mod control_flow;
pub mod globals;
mod react;
mod registry;
mod semantic_analyzers;
mod semantic_services;
pub mod utils;

pub use crate::registry::visit_registry;
use crate::semantic_services::{SemanticModelBuilderVisitor, SemanticModelVisitor};

pub(crate) type JsRuleAction = RuleAction<JsLanguage>;

/// Return the static [MetadataRegistry] for the JS analyzer rules
pub fn metadata() -> &'static MetadataRegistry {
    lazy_static::lazy_static! {
        static ref METADATA: MetadataRegistry = {
            let mut metadata = MetadataRegistry::default();
            visit_registry(&mut metadata);
            metadata
        };
    }

    &*METADATA
}

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action.
/// Additionally, this function takes a `inspect_matcher` function that can be
/// used to inspect the "query matches" emitted by the analyzer before they are
/// processed by the lint rules registry
pub fn analyze_with_inspect_matcher<'a, V, F, B>(
    file_id: FileId,
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    inspect_matcher: V,
    options: &'a AnalyzerOptions,
    mut emit_signal: F,
) -> Option<B>
where
    V: FnMut(&MatchQueryParams<JsLanguage>) + 'a,
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

    let mut registry = RuleRegistry::builder(&filter);
    visit_registry(&mut registry);

    let mut analyzer = Analyzer::new(
        metadata(),
        InspectMatcher::new(registry.build(), inspect_matcher),
        parse_linter_suppression_comment,
        &mut emit_signal,
    );
    analyzer.add_visitor(Phases::Syntax, make_visitor());
    analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default());
    analyzer.add_visitor(Phases::Syntax, SemanticModelBuilderVisitor::new(root));

    analyzer.add_visitor(Phases::Semantic, SemanticModelVisitor);
    analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default());

    analyzer.run(AnalyzerContext {
        file_id,
        root: root.clone(),
        range: filter.range,
        services: ServiceBag::default(),
        options,
    })
}

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action
pub fn analyze<'a, F, B>(
    file_id: FileId,
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    options: &'a AnalyzerOptions,
    emit_signal: F,
) -> Option<B>
where
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_inspect_matcher(file_id, root, filter, |_| {}, options, emit_signal)
}

#[cfg(test)]
mod tests {

    use rome_analyze::{AnalyzerOptions, Never, RuleCategories};
    use rome_console::fmt::{Formatter, Termcolor};
    use rome_console::{markup, Markup};
    use rome_diagnostics::termcolor::NoColor;
    use rome_diagnostics::v2::{Diagnostic, DiagnosticExt, PrintDiagnostic, Severity};
    use rome_diagnostics::{file::FileId, v2::category};
    use rome_js_parser::parse;
    use rome_js_syntax::{SourceType, TextRange, TextSize};

    use crate::{analyze, AnalysisFilter, ControlFlow};

    #[ignore]
    #[test]
    fn quick_test() {
        fn markup_to_string(markup: Markup) -> String {
            let mut buffer = Vec::new();
            let mut write = Termcolor(NoColor::new(&mut buffer));
            let mut fmt = Formatter::new(&mut write);
            fmt.write_markup(markup).unwrap();

            String::from_utf8(buffer).unwrap()
        }

        const SOURCE: &str = r#"<input disabled />
        "#;

        let parsed = parse(SOURCE, FileId::zero(), SourceType::jsx());

        let mut error_ranges: Vec<TextRange> = Vec::new();
        let options = AnalyzerOptions::default();
        analyze(
            FileId::zero(),
            &parsed.tree(),
            AnalysisFilter::default(),
            &options,
            |signal| {
                if let Some(mut diag) = signal.diagnostic() {
                    diag.set_severity(Severity::Warning);
                    error_ranges.push(diag.location().unwrap().span.unwrap());
                    if let Some(action) = signal.action() {
                        let new_code = action.mutation.commit();
                        eprintln!("{new_code}");
                    }
                    let error = diag
                        .with_file_path("example.js")
                        .with_file_source_code(SOURCE);
                    let text = markup_to_string(markup! {
                        {PrintDiagnostic(&error)}
                    });
                    eprintln!("{text}");
                }

                ControlFlow::<Never>::Continue(())
            },
        );

        assert_eq!(error_ranges.as_slice(), &[]);
    }

    #[test]
    fn suppression() {
        const SOURCE: &str = "
            function checkSuppressions1(a, b) {
                a == b;
                // rome-ignore lint(correctness): whole group
                a == b;
                // rome-ignore lint(correctness/noDoubleEquals): single rule
                a == b;
                /* rome-ignore lint(correctness/useWhile): multiple block comments */ /* rome-ignore lint(correctness/noDoubleEquals): multiple block comments */
                a == b;
                // rome-ignore lint(correctness/useWhile): multiple line comments
                // rome-ignore lint(correctness/noDoubleEquals): multiple line comments
                a == b;
                a == b;
            }

            // rome-ignore lint(correctness/noDoubleEquals): do not suppress warning for the whole function
            function checkSuppressions2(a, b) {
                a == b;
            }
        ";

        let parsed = parse(SOURCE, FileId::zero(), SourceType::js_module());

        let mut error_ranges: Vec<TextRange> = Vec::new();
        let options = AnalyzerOptions::default();
        analyze(
            FileId::zero(),
            &parsed.tree(),
            AnalysisFilter::default(),
            &options,
            |signal| {
                if let Some(mut diag) = signal.diagnostic() {
                    diag.set_severity(Severity::Warning);
                    let code = diag.category().unwrap();
                    let location = diag.location().unwrap();

                    if code == category!("lint/correctness/noDoubleEquals") {
                        error_ranges.push(location.span.unwrap());
                    }
                }

                ControlFlow::<Never>::Continue(())
            },
        );

        assert_eq!(
            error_ranges.as_slice(),
            &[
                TextRange::new(TextSize::from(67), TextSize::from(69)),
                TextRange::new(TextSize::from(658), TextSize::from(660)),
                TextRange::new(TextSize::from(853), TextSize::from(855)),
            ]
        );
    }

    #[test]
    fn suppression_syntax() {
        const SOURCE: &str = "
            // rome-ignore lint(correctness/noDoubleEquals): single rule
            a == b;
        ";

        let parsed = parse(SOURCE, FileId::zero(), SourceType::js_module());

        let filter = AnalysisFilter {
            categories: RuleCategories::SYNTAX,
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        analyze(FileId::zero(), &parsed.tree(), filter, &options, |signal| {
            if let Some(mut diag) = signal.diagnostic() {
                diag.set_severity(Severity::Warning);
                let code = diag.category().unwrap();
                panic!("unexpected diagnostic {code:?}");
            }

            ControlFlow::<Never>::Continue(())
        });
    }
}

/// Series of errors encountered when running rules on a file
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum RuleError {
    /// The rule with the specified name replaced the root of the file with a node that is not a valid root for that language.
    ReplacedRootWithNonRootError { rule_name: Cow<'static, str> },
}

impl std::fmt::Display for RuleError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuleError::ReplacedRootWithNonRootError { rule_name } => {
                std::write!(
                    fmt,
                    "the rule '{rule_name}' replaced the root of the file with a non-root node."
                )
            }
        }
    }
}

impl Error for RuleError {}
