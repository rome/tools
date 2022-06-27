#![deny(rustdoc::broken_intra_doc_links)]

use std::ops;

mod categories;
pub mod context;
mod query;
mod registry;
mod rule;
mod services;
mod signals;
mod syntax;
mod visitor;

pub use crate::categories::{ActionCategory, RuleCategories, RuleCategory};
pub use crate::query::{Ast, CannotCreateServicesError, QueryKey, QueryMatch, Queryable};
pub use crate::registry::{LanguageRoot, RuleKey, RuleRegistry};
pub use crate::registry::{Phase, Phases};
pub use crate::rule::{Rule, RuleAction, RuleDiagnostic, RuleMeta};
pub use crate::services::{ServiceBag, ServiceBagData};
pub use crate::signals::{AnalyzerAction, AnalyzerSignal};
pub use crate::syntax::SyntaxVisitor;
pub use crate::visitor::{NodeVisitor, Visitor, VisitorContext};
use rome_diagnostics::file::FileId;
use rome_rowan::{AstNode, Language, SyntaxKind, SyntaxNode, TextRange, WalkEvent};
use rustc_hash::FxHashMap;

/// The analyzer is the main entry point into the `rome_analyze` infrastructure.
/// Its role is to run a collection of [Visitor]s over a syntax tree, with each
/// visitor implementing various analysis over this syntax tree to generate
/// auxiliary data structures as well as emit "query match" events to be
/// processed by lint rules and in turn emit "analyzer signals" in the form of
/// diagnostics, code actions or both
pub struct Analyzer<'a, L: Language, Matcher, Break> {
    /// List of visitors being run by this instance of the analyzer
    visitors: Vec<Box<dyn Visitor<Language = L> + 'a>>,
    /// Executor for the query matches emitted by the visitors
    query_matcher: Matcher,
    /// Language-specific suppression comment parsing function
    parse_suppression_comment: SuppressionParser,
    /// Tracks whether lint rules are currently being fully suppressed
    suppress_all: bool,
    /// Track active suppression comments for nodes in the tree being visited
    node_suppressions: FxHashMap<SyntaxNode<L>, (bool, Vec<RuleKey>)>,
    /// Handles analyzer signals emitted by invidual rules
    emit_signal: SignalHandler<'a, L, Break>,
}

impl<'a, L, Matcher, Break> Analyzer<'a, L, Matcher, Break>
where
    L: Language,
    Matcher: QueryMatcher<L>,
{
    /// Construct a new instance of the analyzer with the given rule registry
    /// and suppression comment parser
    pub fn new(
        query_matcher: Matcher,
        parse_suppression_comment: SuppressionParser,
        emit_signal: SignalHandler<'a, L, Break>,
    ) -> Self {
        Self {
            visitors: Vec::new(),
            query_matcher,
            parse_suppression_comment,
            suppress_all: false,
            node_suppressions: FxHashMap::default(),
            emit_signal,
        }
    }

    pub fn add_visitor<V>(&mut self, visitor: V)
    where
        V: Visitor<Language = L> + 'a,
    {
        self.visitors.push(Box::new(visitor));
    }

    pub fn run(mut self, ctx: &mut VisitorContext<L>) -> Option<Break> {
        for event in ctx.root.syntax().preorder() {
            for visitor in &mut self.visitors {
                visitor.visit(&event, ctx);
            }

            let result = self.flush_queries(ctx, event);
            if let ControlFlow::Break(br) = result {
                return Some(br);
            }
        }

        None
    }

    fn flush_queries(
        &mut self,
        ctx: &mut VisitorContext<L>,
        event: WalkEvent<SyntaxNode<L>>,
    ) -> ControlFlow<Break> {
        // Find a safe position corresponding to this event in the source, all
        // query matches emitted before this position can safely be flushed
        let (cutoff, allow_equal) = match &event {
            // Entering a node: flush all matches before the start of that node
            WalkEvent::Enter(event) => (event.text_range().start(), false),
            // Leaving a node: flush all matches before the end of that node
            WalkEvent::Leave(event) => (event.text_range().end(), true),
        };

        // Flush matches from the queue until the cutoff point is reached
        while let Some(query) = ctx.match_queue.front() {
            // Compare the position of the query match to the cutoff point
            match query {
                QueryMatch::ControlFlowGraph(_) => {
                    // TODO
                }
                QueryMatch::Syntax(node) => {
                    let start = node.text_range().start();
                    if (allow_equal && start > cutoff) || (!allow_equal && start >= cutoff) {
                        break;
                    }
                }
            }

            // Run the query match if lints are not currently being suppressed
            if !self.suppress_all {
                let mut break_value = None;

                let result = self.query_matcher.match_query(
                    ctx.phase,
                    ctx.file_id,
                    &ctx.root,
                    query,
                    &ctx.services,
                    |signal| match (self.emit_signal)(signal) {
                        ControlFlow::Continue(()) => ControlFlow::Continue(()),
                        ControlFlow::Break(value) => {
                            break_value = Some(value);
                            ControlFlow::Break(())
                        }
                    },
                );

                if result.is_break() {
                    // SAFETY: `match_query` should only return `Break` values
                    // emitted by the signal handler and stored in `break_value`
                    return ControlFlow::Break(break_value.unwrap());
                }
            }

            // SAFETY: This removes `query` from the queue, it is known to
            // exist since the `while let Some` block was entered
            ctx.match_queue.pop_front().unwrap();
        }

        // Process the incoming event
        let node = match event {
            // Entering a node: move on to processing suppression comments
            WalkEvent::Enter(node) => node,
            // Leaving a node: if it has associated suppression comments,
            // remove them from the suppression state
            WalkEvent::Leave(node) => {
                if let Some((suppress_all, suppressions)) = self.node_suppressions.remove(&node) {
                    if suppress_all {
                        self.suppress_all = false;
                    }

                    self.query_matcher.remove_suppressions(suppressions);
                }

                return ControlFlow::Continue(());
            }
        };

        // Syntax lists and root nodes cannot have suppression comments
        if !is_suppression_node(&node) {
            return ControlFlow::Continue(());
        }

        // Lookup the first direct child token of this node, ensuring the start
        // of its range coincides with the start of the node (this ensures this
        // was actually the first token in the node and it was not preceeded by
        // any non-empty node child)
        let node_range = node.text_range();
        let first_token = node
            .tokens()
            .next()
            .filter(|token| token.text_range().start() == node_range.start());

        let first_token = match first_token {
            Some(token) => token,
            None => return ControlFlow::Continue(()),
        };

        // Read the leading trivia of the token for comment pieces, and try to
        // parse those as suppression comments
        let comments = first_token
            .leading_trivia()
            .pieces()
            .filter_map(|trivia| trivia.as_comments());

        let mut suppress_all = false;
        let mut suppressions = Vec::new();

        'comments: for comment in comments {
            for (category, rule) in (self.parse_suppression_comment)(comment.text()) {
                if category != "lint" {
                    continue;
                }

                if let Some(rule) = rule {
                    if let Some(rule) = self.query_matcher.insert_suppression(rule) {
                        suppressions.push(rule);
                    }
                } else {
                    if !self.suppress_all {
                        self.suppress_all = true;
                        suppress_all = true;
                    }

                    // If this if a "suppress all lints" comment, no need to
                    // parse anything else
                    break 'comments;
                }
            }
        }

        // If the node didn't have any new suppression we don't need to do any
        // further processing
        if !suppress_all && suppressions.is_empty() {
            return ControlFlow::Continue(());
        }

        // Walk up the syntax tree to find the highest ancestor node these
        // suppressions can be attached to
        let ancestor = node
            .ancestors()
            .take_while(is_suppression_node)
            .filter(|node| node.text_range().start() == node_range.start())
            .last();

        if let Some(ancestor) = ancestor {
            self.node_suppressions
                .insert(ancestor, (suppress_all, suppressions));
        }

        ControlFlow::Continue(())
    }
}

/// Signature for a suppression comment parser function
type SuppressionParser = fn(&str) -> Vec<(&str, Option<&str>)>;

type SignalHandler<'a, L, Break> = &'a mut dyn FnMut(&dyn AnalyzerSignal<L>) -> ControlFlow<Break>;

/// Returns `true` if this node can have suppression comments attached (is not
/// a list or a root node)
fn is_suppression_node<L: Language>(node: &SyntaxNode<L>) -> bool {
    let kind = node.kind();
    !kind.is_list() && !<L::Root>::can_cast(kind)
}

pub trait QueryMatcher<L: Language> {
    /// Execute a single query match
    fn match_query<'a>(
        &mut self,
        phase: Phases,
        file_id: FileId,
        root: &'a L::Root,
        query: &'a QueryMatch<L>,
        services: &ServiceBag,
        emit_signal: impl FnMut(&dyn AnalyzerSignal<L>) -> ControlFlow<()>,
    ) -> ControlFlow<()>;

    /// Try to add a rule to the suppressed rule set. Return [Some] with a
    /// unique identifier for the rule if the name is known to the matcher and
    /// was added successfully
    fn insert_suppression(&mut self, name: &str) -> Option<RuleKey>;
    /// Remove a number of lint rules from the suppressed rule set
    fn remove_suppressions(&mut self, suppressions: impl IntoIterator<Item = RuleKey>);
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
/// `analyze` when the provided callback never breaks
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
/// reason the `analyze` function returns an `Option<B>` that's set to
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
