mod categories;
pub mod context;
mod registry;
mod signals;

pub use crate::categories::{ActionCategory, RuleCategories, RuleCategory};
pub use crate::registry::{LanguageRoot, Rule, RuleAction, RuleDiagnostic, RuleRegistry};
pub use crate::signals::{AnalyzerAction, AnalyzerSignal};
use rome_diagnostics::file::FileId;
use rome_rowan::{AstNode, Language, SyntaxNode, TextRange, WalkEvent};
use std::ops;

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

impl AnalysisFilter<'_> {
    /// Return `true` if the rule `R` matches this filter
    pub fn match_rule<R: Rule>(&self) -> bool {
        self.categories.contains(R::CATEGORY.into())
            && self.rules.map_or(true, |rules| rules.contains(&R::NAME))
    }
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

/// The [Analyzer] is intended is an executor for a set of rules contained in a
/// [RuleRegistry]: these rules can query the syntax tree and emit [AnalyzerSignal]
/// objects in response to a query match, with a signal being a generic object
/// containing a diagnostic, a code action or both.
pub struct Analyzer<L: Language> {
    registry: RuleRegistry<L>,
    has_suppressions: fn(&SyntaxNode<L>) -> bool,
}

impl<L: Language> Analyzer<L> {
    /// Create a new instance of the analyzer from a registry instance, and a
    /// language-specific suppression parser
    pub fn new(registry: RuleRegistry<L>, has_suppressions: fn(&SyntaxNode<L>) -> bool) -> Self {
        Self {
            registry,
            has_suppressions,
        }
    }

    /// Run the analyzer on the provided `root`: this process will use the given `filter`
    /// to selectively restrict analysis to specific rules / a specific source range,
    /// then call the `callback` when an analysis rule emits a diagnostic or action
    pub fn analyze<B>(
        self,
        file_id: FileId,
        root: &LanguageRoot<L>,
        range: Option<TextRange>,
        mut callback: impl FnMut(&dyn AnalyzerSignal<L>) -> ControlFlow<B>,
    ) -> Option<B> {
        let mut iter = root.syntax().preorder();
        while let Some(event) = iter.next() {
            let node = match event {
                WalkEvent::Enter(node) => node,
                WalkEvent::Leave(_) => continue,
            };

            if let Some(range) = range {
                if node.text_range().ordering(range).is_ne() {
                    iter.skip_subtree();
                    continue;
                }
            }

            if (self.has_suppressions)(&node) {
                iter.skip_subtree();
                continue;
            }

            if let ControlFlow::Break(b) = self.registry.analyze(file_id, root, node, &mut callback)
            {
                return Some(b);
            }
        }

        None
    }
}
