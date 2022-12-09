pub use crate::registry::visit_registry;
use crate::semantic_services::{SemanticModelBuilderVisitor, SemanticModelVisitor};
use crate::suppression_action::apply_suppression_comment;
use control_flow::make_visitor;
use rome_analyze::context::ServiceBagRuleOptionsWrapper;
use rome_analyze::options::OptionsDeserializationDiagnostic;
use rome_analyze::{
    AnalysisFilter, Analyzer, AnalyzerContext, AnalyzerOptions, AnalyzerSignal, ControlFlow,
    DeserializableRuleOptions, InspectMatcher, LanguageRoot, MatchQueryParams, MetadataRegistry,
    Phases, RuleAction, RuleRegistry, ServiceBag, SuppressionKind, SyntaxVisitor,
};
use rome_aria::{AriaProperties, AriaRoles};
use rome_diagnostics::{category, Diagnostic, FileId};
use rome_js_syntax::suppression::SuppressionDiagnostic;
use rome_js_syntax::{suppression::parse_suppression_comment, JsLanguage};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{borrow::Cow, error::Error};

mod analyzers;
mod aria_analyzers;
mod aria_services;
mod assists;
mod ast_utils;
mod control_flow;
pub mod globals;
mod react;
mod registry;
mod semantic_analyzers;
mod semantic_services;
mod suppression_action;
mod syntax;
pub mod utils;

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

    &METADATA
}

pub struct RulesConfigurator<'a> {
    options: &'a AnalyzerOptions,
    services: &'a mut ServiceBag,
    diagnostics: Vec<OptionsDeserializationDiagnostic>,
}

impl<'a, L: rome_rowan::Language + Default> rome_analyze::RegistryVisitor<L>
    for RulesConfigurator<'a>
{
    fn record_rule<R>(&mut self)
    where
        R: rome_analyze::Rule + 'static,
        R::Query: rome_analyze::Queryable<Language = L>,
        <R::Query as rome_analyze::Queryable>::Output: Clone,
    {
        let rule_key = rome_analyze::RuleKey::rule::<R>();
        let options = if let Some(options) = self.options.configuration.rules.get_rule(&rule_key) {
            let value = options.value();
            match <R::Options as DeserializableRuleOptions>::try_from(value.clone()) {
                Ok(result) => result,
                Err(error) => {
                    let err = OptionsDeserializationDiagnostic::new(
                        rule_key.rule_name(),
                        value.to_string(),
                        error,
                    );
                    self.diagnostics.push(err);
                    <R::Options as Default>::default()
                }
            }
        } else {
            <R::Options as Default>::default()
        };

        self.services
            .insert_service(ServiceBagRuleOptionsWrapper::<R>(options));
    }
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
    fn parse_linter_suppression_comment(
        text: &str,
    ) -> Vec<Result<SuppressionKind, SuppressionDiagnostic>> {
        let mut result = Vec::new();

        for comment in parse_suppression_comment(text) {
            let categories = match comment {
                Ok(comment) => comment.categories,
                Err(err) => {
                    result.push(Err(err));
                    continue;
                }
            };

            for (key, value) in categories {
                if key == category!("lint") {
                    if let Some(value) = value {
                        result.push(Ok(SuppressionKind::MaybeLegacy(value)));
                    } else {
                        result.push(Ok(SuppressionKind::Everything));
                    }
                } else {
                    let category = key.name();
                    if let Some(rule) = category.strip_prefix("lint/") {
                        result.push(Ok(SuppressionKind::Rule(rule)));
                    }
                }
            }
        }

        result
    }

    let mut registry = RuleRegistry::builder(&filter);
    visit_registry(&mut registry);

    // Parse rule options
    let mut services = ServiceBag::default();
    let mut configurator = RulesConfigurator {
        options,
        services: &mut services,
        diagnostics: vec![],
    };
    visit_registry(&mut configurator);

    // Bail if we can't parse a rule option
    if !configurator.diagnostics.is_empty() {
        for diagnostic in configurator.diagnostics {
            emit_signal(&diagnostic);
        }
        return None;
    }

    let mut analyzer = Analyzer::new(
        metadata(),
        InspectMatcher::new(registry.build(), inspect_matcher),
        parse_linter_suppression_comment,
        apply_suppression_comment,
        &mut emit_signal,
    );
    analyzer.add_visitor(Phases::Syntax, make_visitor());
    analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default());
    analyzer.add_visitor(Phases::Syntax, SemanticModelBuilderVisitor::new(root));

    analyzer.add_visitor(Phases::Semantic, SemanticModelVisitor);
    analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default());

    services.insert_service(Arc::new(AriaRoles::default()));
    services.insert_service(Arc::new(AriaProperties::default()));
    analyzer.run(AnalyzerContext {
        file_id,
        root: root.clone(),
        range: filter.range,
        services,
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
    use rome_analyze::{AnalyzerOptions, Never, RuleCategories, RuleFilter};
    use rome_console::fmt::{Formatter, Termcolor};
    use rome_console::{markup, Markup};
    use rome_diagnostics::termcolor::NoColor;
    use rome_diagnostics::{category, location::FileId};
    use rome_diagnostics::{Diagnostic, DiagnosticExt, PrintDiagnostic, Severity};
    use rome_js_parser::parse;
    use rome_js_syntax::{SourceType, TextRange, TextSize};
    use std::slice;

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

        const SOURCE: &str = r#"something.forEach((Element, index) => {
    return <List
        ><div key={index}>foo</div>
    </List>;
})"#;

        let parsed = parse(SOURCE, FileId::zero(), SourceType::jsx());

        let mut error_ranges: Vec<TextRange> = Vec::new();
        let options = AnalyzerOptions::default();
        let rule_filter = RuleFilter::Rule("suspicious", "noArrayIndexKey");
        analyze(
            FileId::zero(),
            &parsed.tree(),
            AnalysisFilter {
                enabled_rules: Some(slice::from_ref(&rule_filter)),
                ..AnalysisFilter::default()
            },
            &options,
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    error_ranges.push(diag.location().span.unwrap());
                    let error = diag
                        .with_severity(Severity::Warning)
                        .with_file_path("ahahah")
                        .with_file_source_code(SOURCE);
                    let text = markup_to_string(markup! {
                        {PrintDiagnostic::verbose(&error)}
                    });
                    eprintln!("{text}");
                }

                for action in signal.actions() {
                    let new_code = action.mutation.commit();
                    eprintln!("{new_code}");
                }

                ControlFlow::<Never>::Continue(())
            },
        );

        // assert_eq!(error_ranges.as_slice(), &[]);
    }

    #[test]
    fn suppression() {
        const SOURCE: &str = "
            function checkSuppressions1(a, b) {
                a == b;
                // rome-ignore lint/suspicious:whole group
                a == b;
                // rome-ignore lint/suspicious/noDoubleEquals: single rule
                a == b;
                /* rome-ignore lint/style/useWhile: multiple block comments */ /* rome-ignore lint/suspicious/noDoubleEquals: multiple block comments */
                a == b;
                // rome-ignore lint/style/useWhile: multiple line comments
                // rome-ignore lint/suspicious/noDoubleEquals: multiple line comments
                a == b;
                a == b;
            }

            // rome-ignore lint/suspicious/noDoubleEquals: do not suppress warning for the whole function
            function checkSuppressions2(a, b) {
                a == b;
            }

            function checkSuppressions3(a, b) {
                a == b;
                // rome-ignore lint(suspicious): whole group
                a == b;
                // rome-ignore lint(suspicious/noDoubleEquals): single rule
                a == b;
                /* rome-ignore lint(style/useWhile): multiple block comments */ /* rome-ignore lint(suspicious/noDoubleEquals): multiple block comments */
                a == b;
                // rome-ignore lint(style/useWhile): multiple line comments
                // rome-ignore lint(suspicious/noDoubleEquals): multiple line comments
                a == b;
                a == b;
            }

            // rome-ignore lint(suspicious/noDoubleEquals): do not suppress warning for the whole function
            function checkSuppressions4(a, b) {
                a == b;
            }

            function checkSuppressions5() {
                // rome-ignore format explanation
                // rome-ignore format(:
                // rome-ignore (value): explanation
                // rome-ignore unknown: explanation
            }
        ";

        let parsed = parse(SOURCE, FileId::zero(), SourceType::js_module());

        let mut lint_ranges: Vec<TextRange> = Vec::new();
        let mut parse_ranges: Vec<TextRange> = Vec::new();
        let mut warn_ranges: Vec<TextRange> = Vec::new();

        let options = AnalyzerOptions::default();
        analyze(
            FileId::zero(),
            &parsed.tree(),
            AnalysisFilter::default(),
            &options,
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    let span = diag.get_span();
                    let error = diag
                        .with_severity(Severity::Warning)
                        .with_file_path(FileId::zero())
                        .with_file_source_code(SOURCE);

                    let code = error.category().unwrap();
                    if code == category!("lint/suspicious/noDoubleEquals") {
                        lint_ranges.push(span.unwrap());
                    }

                    if code == category!("suppressions/parse") {
                        parse_ranges.push(span.unwrap());
                    }

                    if code == category!("suppressions/deprecatedSyntax") {
                        assert!(signal.actions().len() > 0);
                        warn_ranges.push(span.unwrap());
                    }
                }

                ControlFlow::<Never>::Continue(())
            },
        );

        assert_eq!(
            lint_ranges.as_slice(),
            &[
                TextRange::new(TextSize::from(67), TextSize::from(69)),
                TextRange::new(TextSize::from(635), TextSize::from(637)),
                TextRange::new(TextSize::from(828), TextSize::from(830)),
                TextRange::new(TextSize::from(915), TextSize::from(917)),
                TextRange::new(TextSize::from(1490), TextSize::from(1492)),
                TextRange::new(TextSize::from(1684), TextSize::from(1686)),
            ]
        );

        assert_eq!(
            parse_ranges.as_slice(),
            &[
                TextRange::new(TextSize::from(1787), TextSize::from(1798)),
                TextRange::new(TextSize::from(1837), TextSize::from(1838)),
                TextRange::new(TextSize::from(1870), TextSize::from(1871)),
                TextRange::new(TextSize::from(1922), TextSize::from(1929)),
            ]
        );

        assert_eq!(
            warn_ranges.as_slice(),
            &[
                TextRange::new(TextSize::from(937), TextSize::from(981)),
                TextRange::new(TextSize::from(1022), TextSize::from(1081)),
                TextRange::new(TextSize::from(1122), TextSize::from(1185)),
                TextRange::new(TextSize::from(1186), TextSize::from(1260)),
                TextRange::new(TextSize::from(1301), TextSize::from(1360)),
                TextRange::new(TextSize::from(1377), TextSize::from(1447)),
                TextRange::new(TextSize::from(1523), TextSize::from(1617)),
            ]
        );
    }

    #[test]
    fn suppression_syntax() {
        const SOURCE: &str = "
            // rome-ignore lint/suspicious/noDoubleEquals: single rule
            a == b;
        ";

        let parsed = parse(SOURCE, FileId::zero(), SourceType::js_module());

        let filter = AnalysisFilter {
            categories: RuleCategories::SYNTAX,
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        analyze(FileId::zero(), &parsed.tree(), filter, &options, |signal| {
            if let Some(diag) = signal.diagnostic() {
                let code = diag.category().unwrap();
                if code != category!("suppressions/unused") {
                    panic!("unexpected diagnostic {code:?}");
                }
            }

            ControlFlow::<Never>::Continue(())
        });
    }
}

/// Series of errors encountered when running rules on a file
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum RuleError {
    /// The rule with the specified name replaced the root of the file with a node that is not a valid root for that language.
    ReplacedRootWithNonRootError {
        rule_name: Option<(Cow<'static, str>, Cow<'static, str>)>,
    },
}

impl Diagnostic for RuleError {}

impl std::fmt::Display for RuleError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuleError::ReplacedRootWithNonRootError {
                rule_name: Some((group, rule)),
            } => {
                std::write!(
                    fmt,
                    "the rule '{group}/{rule}' replaced the root of the file with a non-root node."
                )
            }
            RuleError::ReplacedRootWithNonRootError { rule_name: None } => {
                std::write!(
                    fmt,
                    "a code action replaced the root of the file with a non-root node."
                )
            }
        }
    }
}

impl rome_console::fmt::Display for RuleError {
    fn fmt(&self, fmt: &mut rome_console::fmt::Formatter) -> std::io::Result<()> {
        match self {
            RuleError::ReplacedRootWithNonRootError {
                rule_name: Some((group, rule)),
            } => {
                std::write!(
                    fmt,
                    "the rule '{group}/{rule}' replaced the root of the file with a non-root node."
                )
            }
            RuleError::ReplacedRootWithNonRootError { rule_name: None } => {
                std::write!(
                    fmt,
                    "a code action replaced the root of the file with a non-root node."
                )
            }
        }
    }
}

impl Error for RuleError {}
