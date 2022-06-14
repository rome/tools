use std::ops;

use context::RuleContextServiceBag;
use rome_diagnostics::file::FileId;
use rome_js_syntax::{
    suppression::{has_suppressions_category, SuppressionCategory},
    JsLanguage, TextRange, WalkEvent,
};
use rome_rowan::AstNode;

mod analyzers;
mod assists;
mod categories;
mod context;
mod registry;
mod signals;

pub use crate::categories::{ActionCategory, RuleCategories, RuleCategory};
use crate::registry::{LanguageRoot, RuleRegistry};
pub use crate::signals::{AnalyzerAction, AnalyzerSignal};

pub(crate) type LanguageOfRule<TRule> = <<TRule as registry::Rule>::Query as AstNode>::Language;

/// Allows filtering the list of rules that will be executed in a run of the analyzer,
/// and at what source code range signals (diagnostics or actions) may be raised
#[derive(Default, Clone, Copy)]
pub struct AnalysisFilter<'a> {
    /// Only allow rules with these categories to emit signals
    pub categories: RuleCategories,
    /// Only allow rules with these names to emit signals
    pub rules: Option<&'a [&'a str]>,
    /// Only emit signals matching this text range
    pub range: Option<TextRange>,
}

/// Utility type to be used as a default value for the `B` generic type on
/// [analyze] when the provided callback never breaks
///
/// This should eventually get replaced with the `!` type when it gets stabilized
pub enum Never {}

/// Type alias of [ops::ControlFlow] with the `B` generic type defaulting to [Never]
///
/// By default the analysis loop never breaks, so it behaves mostly like
/// `let b = loop {};` and has a "break type" of `!` (the `!` type isn't stable
/// yet so I'm using an empty enum instead but they're identical for this
/// purpose)
///
/// In practice it's not really a `loop` but a `for` because it's iterating on
/// all nodes in the syntax tree, so when it reaches the end of the iterator
/// the loop will exit but without producing a value of type `B`: for this
/// reason the [analyze] function returns an `Option<B>` that's set to
/// `Some(B)` if the callback did break, and `None` if the analysis reached the
/// end of the file.
///
/// Most consumers of the analyzer will want to analyze the entire file at once
/// and never break, so using [Never] as the type of `B` in this case lets the
/// compiler know the `ControlFlow::Break` branch will never be taken and can
/// be optimized out, as well as completely remove the `return Some` case
/// (`Option<Never>` has a size of 0 and can be elided, while `Option<()>` has
/// a size of 1 as it still need to store a discriminant)
pub type ControlFlow<B = Never> = ops::ControlFlow<B>;

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call the `callback` when an analysis rule emits a diagnostic or action
pub fn analyze<B>(
    file_id: FileId,
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    mut callback: impl FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B>,
) -> Option<B> {
    let services = RuleContextServiceBag::new(root.clone());
    let registry = RuleRegistry::new(services, &filter);

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

        if let ControlFlow::Break(b) = registry.analyze(file_id, root, node, &mut callback) {
            return Some(b);
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use rome_js_parser::parse;
    use rome_js_syntax::SourceType;

    use crate::{analyze, AnalysisFilter, ControlFlow, Never};

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
