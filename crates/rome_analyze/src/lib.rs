use std::ops;

mod categories;
pub mod context;
mod query;
mod registry;
mod rule;
mod signals;
mod visitor;

pub use crate::categories::{ActionCategory, RuleCategories, RuleCategory};
pub use crate::query::{Ast, QueryKey, QueryMatch, Queryable};
pub use crate::registry::{LanguageRoot, RuleRegistry};
pub use crate::rule::{Rule, RuleAction, RuleDiagnostic, RuleMeta};
pub use crate::signals::{AnalyzerAction, AnalyzerSignal};
pub use crate::visitor::{NodeVisitor, Visitor, VisitorContext};
use rome_rowan::{AstNode, Language, SyntaxNode, TextRange, WalkEvent};

/// The analyzer is the main entry point into the `rome_analyze` infrastructure.
/// Its role is to run a collection of [Visitor]s over a syntax tree, with each
/// visitor implementing various analysis over this syntax tree to generate
/// auxiliary data structures as well as emit "query match" events to be
/// processed by lint rules and in turn emit "analyzer signals" in the form of
/// diagnostics, code actions or both
pub struct Analyzer<L, B> {
    visitors: Vec<Box<dyn Visitor<B, Language = L>>>,
}

impl<L: Language, B> Analyzer<L, B> {
    pub fn empty() -> Self {
        Self {
            visitors: Vec::new(),
        }
    }

    pub fn add_visitor<V>(&mut self, visitor: V)
    where
        V: Visitor<B, Language = L> + 'static,
    {
        self.visitors.push(Box::new(visitor));
    }

    pub fn run(mut self, ctx: &mut VisitorContext<L, B>) -> Option<B> {
        for event in ctx.root.syntax().preorder() {
            for visitor in &mut self.visitors {
                if let ControlFlow::Break(br) = visitor.visit(&event, ctx) {
                    return Some(br);
                }
            }
        }

        None
    }
}

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

/// The [SyntaxVisitor] is the simplest form of visitor implemented for the
/// analyzer, it simply broadcast each [WalkEvent::Enter] as a query match
/// event for the [SyntaxNode] being entered
pub struct SyntaxVisitor<L: Language, F> {
    has_suppressions: F,
    skip_subtree: Option<SyntaxNode<L>>,
}

impl<L: Language, F> SyntaxVisitor<L, F>
where
    F: Fn(&SyntaxNode<L>) -> bool,
{
    pub fn new(has_suppressions: F) -> Self {
        Self {
            has_suppressions,
            skip_subtree: None,
        }
    }
}

impl<L: Language, F, B> Visitor<B> for SyntaxVisitor<L, F>
where
    F: Fn(&SyntaxNode<L>) -> bool,
{
    type Language = L;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        ctx: &mut VisitorContext<L, B>,
    ) -> ControlFlow<B> {
        let node = match event {
            WalkEvent::Enter(node) => node,
            WalkEvent::Leave(node) => {
                if let Some(skip_subtree) = &self.skip_subtree {
                    if skip_subtree == node {
                        self.skip_subtree.take();
                    }
                }

                return ControlFlow::Continue(());
            }
        };

        if self.skip_subtree.is_some() {
            return ControlFlow::Continue(());
        }

        if let Some(range) = ctx.range {
            if node.text_range().ordering(range).is_ne() {
                self.skip_subtree = Some(node.clone());
                return ControlFlow::Continue(());
            }
        }

        if (self.has_suppressions)(node) {
            self.skip_subtree = Some(node.clone());
            return ControlFlow::Continue(());
        }

        let query = QueryMatch::Syntax(node.clone());
        ctx.match_query(&query)
    }
}
