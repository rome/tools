use control_flow::make_visitor;
use rome_analyze::{
    AnalysisFilter, Analyzer, AnalyzerContext, AnalyzerSignal, ControlFlow, InspectMatcher,
    LanguageRoot, MatchQueryParams, Phases, RegistryRuleMetadata, RuleAction, ServiceBag,
    SyntaxVisitor,
};
use rome_diagnostics::file::FileId;
use rome_js_syntax::{
    suppression::{parse_suppression_comment, SuppressionCategory},
    JsLanguage,
};
use std::{borrow::Cow, error::Error};

mod analyzers;
mod assists;
mod control_flow;
mod react;
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
/// then call `emit_signal` when an analysis rule emits a diagnostic or action.
/// Additionally, this function takes a `inspect_matcher` function that can be
/// used to inspect the "query matches" emitted by the analyzer before they are
/// processed by the lint rules registry
pub fn analyze_with_inspect_matcher<'a, V, F, B>(
    file_id: FileId,
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    inspect_matcher: V,
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

    let mut analyzer = Analyzer::new(
        InspectMatcher::new(build_registry(&filter), inspect_matcher),
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

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action
pub fn analyze<'a, F, B>(
    file_id: FileId,
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    emit_signal: F,
) -> Option<B>
where
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_inspect_matcher(file_id, root, filter, |_| {}, emit_signal)
}

#[cfg(test)]
mod tests {

    use rome_analyze::Never;
    use rome_console::codespan::Severity;
    use rome_diagnostics::file::FileId;
    use rome_js_parser::parse;
    use rome_js_syntax::{SourceType, TextRange, TextSize};

    use crate::{analyze, AnalysisFilter, ControlFlow};

    #[ignore]
    #[test]
    fn quick_test() {
        const SOURCE: &str = r#"
if (true) {
  console.log("true");
} else
  console.log("false"); // comment

        "#;

        let parsed = parse(SOURCE, FileId::zero(), SourceType::jsx());

        let mut error_ranges = Vec::new();
        analyze(
            FileId::zero(),
            &parsed.tree(),
            AnalysisFilter::default(),
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    let diag = diag.into_diagnostic(Severity::Warning);
                    let primary = diag.primary.as_ref().unwrap();

                    error_ranges.push(primary.span.range);
                }

                if let Some(action) = signal.action() {
                    let new_code = action.mutation.commit();

                    eprintln!("{new_code}");
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

        let mut error_ranges = Vec::new();
        analyze(
            FileId::zero(),
            &parsed.tree(),
            AnalysisFilter::default(),
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    let diag = diag.into_diagnostic(Severity::Warning);
                    let code = diag.code.as_deref().unwrap();
                    let primary = diag.primary.as_ref().unwrap();

                    if code == "lint/correctness/noDoubleEquals" {
                        error_ranges.push(primary.span.range);
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
}

/// Series of errors encountered when running rules on a file
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
