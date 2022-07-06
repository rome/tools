#![deny(rustdoc::broken_intra_doc_links)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops;

mod categories;
pub mod context;
mod matcher;
mod query;
mod registry;
mod rule;
mod services;
mod signals;
mod syntax;
mod visitor;

pub use crate::categories::{ActionCategory, RuleCategories, RuleCategory};
pub use crate::matcher::{QueryMatcher, RuleKey, SignalEntry};
pub use crate::query::{Ast, CannotCreateServicesError, QueryKey, QueryMatch, Queryable};
pub use crate::registry::{LanguageRoot, RuleRegistry};
pub use crate::registry::{Phase, Phases};
pub use crate::rule::{Rule, RuleAction, RuleDiagnostic, RuleMeta};
pub use crate::services::{ServiceBag, ServiceBagData};
pub use crate::signals::{AnalyzerAction, AnalyzerSignal};
pub use crate::syntax::SyntaxVisitor;
pub use crate::visitor::{NodeVisitor, Visitor, VisitorContext};
use rome_diagnostics::file::FileId;
use rome_rowan::{
    AstNode, Direction, Language, SyntaxElement, SyntaxToken, TextRange, TextSize, TriviaPieceKind,
    WalkEvent,
};

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
    /// Queue for pending analyzer signals
    signal_queue: BinaryHeap<SignalEntry<L>>,
    /// Language-specific suppression comment parsing function
    parse_suppression_comment: SuppressionParser,
    /// Line index at the current position of the traversal
    line_index: usize,
    /// Track active suppression comments per-line, ordered by line index
    line_suppressions: Vec<LineSuppression>,
    /// Handles analyzer signals emitted by invidual rules
    emit_signal: SignalHandler<'a, L, Break>,
}

/// Single entry for a suppression comment in the `line_suppressions` buffer
#[derive(Debug)]
struct LineSuppression {
    /// Line index this comment is suppressing lint rules for
    line_index: usize,
    /// Range of source text this comment is suppressing lint rules for
    text_range: TextRange,
    /// Set to true if this comment has set the `suppress_all` flag to true
    /// (must be restored to false on expiration)
    suppress_all: bool,
    /// List of all the rules this comment has started suppressing (must be
    /// removed from the suppressed set on expiration)
    suppressed_rules: Vec<RuleKey>,
}

pub struct AnalyzerContext<L: Language> {
    pub phase: Phases,
    pub file_id: FileId,
    pub root: LanguageRoot<L>,
    pub services: ServiceBag,
    pub range: Option<TextRange>,
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
            signal_queue: BinaryHeap::new(),
            parse_suppression_comment,
            line_index: 0,
            line_suppressions: Vec::new(),
            emit_signal,
        }
    }

    pub fn add_visitor<V>(&mut self, visitor: V)
    where
        V: Visitor<Language = L> + 'a,
    {
        self.visitors.push(Box::new(visitor));
    }

    pub fn run(mut self, ctx: AnalyzerContext<L>) -> Option<Break> {
        let iter = ctx.root.syntax().preorder_with_tokens(Direction::Next);
        for event in iter {
            let node_event = match event {
                WalkEvent::Enter(SyntaxElement::Node(node)) => WalkEvent::Enter(node),
                WalkEvent::Leave(SyntaxElement::Node(node)) => WalkEvent::Leave(node),

                // If this is a token enter event, process its text content
                WalkEvent::Enter(SyntaxElement::Token(token)) => {
                    let result = self.flush_matches(token);
                    if let ControlFlow::Break(br) = result {
                        return Some(br);
                    }

                    continue;
                }
                WalkEvent::Leave(SyntaxElement::Token(_)) => {
                    continue;
                }
            };

            // If this is a node event pass it to the visitors
            for visitor in &mut self.visitors {
                let ctx = VisitorContext {
                    phase: ctx.phase,
                    file_id: ctx.file_id,
                    root: &ctx.root,
                    services: &ctx.services,
                    range: ctx.range,
                    query_matcher: &mut self.query_matcher,
                    signal_queue: &mut self.signal_queue,
                };

                visitor.visit(&node_event, ctx);
            }
        }

        None
    }

    /// Process the text for a single token, parsing suppression comments and
    /// flushing pending query matches
    fn flush_matches(&mut self, token: SyntaxToken<L>) -> ControlFlow<Break> {
        // Process the content of the token for comments and newline
        for piece in token.leading_trivia().pieces() {
            if matches!(
                piece.kind(),
                TriviaPieceKind::Newline
                    | TriviaPieceKind::MultiLineComment
                    | TriviaPieceKind::Skipped
            ) {
                self.bump_line_index(piece.text(), piece.text_range());
            }

            if let Some(comment) = piece.as_comments() {
                self.handle_comment(comment.text(), piece.text_range());
            }
        }

        self.bump_line_index(token.text_trimmed(), token.text_trimmed_range());

        for piece in token.trailing_trivia().pieces() {
            if matches!(
                piece.kind(),
                TriviaPieceKind::Newline
                    | TriviaPieceKind::MultiLineComment
                    | TriviaPieceKind::Skipped
            ) {
                self.bump_line_index(piece.text(), piece.text_range());
            }

            if let Some(comment) = piece.as_comments() {
                self.handle_comment(comment.text(), piece.text_range());
            }
        }

        // Flush signals from the queue until the end of the current token is reached
        let cutoff = token.text_range().end();
        while let Some(entry) = self.signal_queue.peek() {
            let start = entry.text_range.start();
            if start >= cutoff {
                break;
            }

            // Search for an active suppression comment covering the range of this signal
            let index = self.line_suppressions.binary_search_by(|suppression| {
                if suppression.text_range.end() < entry.text_range.start() {
                    Ordering::Less
                } else if entry.text_range.end() < suppression.text_range.start() {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            let suppression = index.ok().map(|index| &self.line_suppressions[index]);

            // If the line index for the matched suppression is the current one,
            // its text range is still pending: stop signals processing now and
            // try again later
            if let Some(suppression) = suppression {
                if suppression.line_index == self.line_index {
                    break;
                }
            }

            let is_suppressed = suppression.map_or(false, |suppression| {
                suppression.suppress_all || suppression.suppressed_rules.contains(&entry.rule)
            });

            // Emit the signal if the rule that created it is not currently being suppressed
            if !is_suppressed {
                (self.emit_signal)(&*entry.signal)?;
            }

            // SAFETY: This removes `query` from the queue, it is known to
            // exist since the `while let Some` block was entered
            self.signal_queue.pop().unwrap();
        }

        ControlFlow::Continue(())
    }

    fn handle_comment(&mut self, text: &str, range: TextRange) {
        let mut suppress_all = false;
        let mut suppressions = Vec::new();

        for (category, rule) in (self.parse_suppression_comment)(text) {
            if category != "lint" {
                continue;
            }

            if let Some(rule) = rule {
                if let Some(rule) = self.query_matcher.find_rule(rule) {
                    suppressions.push(rule);
                }
            } else {
                suppressions.clear();
                suppress_all = true;
                // If this if a "suppress all lints" comment, no need to
                // parse anything else
                break;
            }
        }

        if !suppress_all && suppressions.is_empty() {
            return;
        }

        // Suppression comments apply to the next line
        let line_index = self.line_index + 1;

        // Ensure proper ordering of the `line_suppressions` buffer
        if let Some(suppression) = self.line_suppressions.last() {
            assert!(suppression.line_index < line_index);
        }

        let entry = LineSuppression {
            line_index,
            text_range: range,
            suppress_all,
            suppressed_rules: suppressions,
        };

        self.line_suppressions.push(entry);
    }

    /// Check a piece of source text (token or trivia) for line breaks and
    /// increment the line index accordingly, extending the range of the
    /// current suppression as required
    fn bump_line_index(&mut self, text: &str, range: TextRange) {
        let mut did_match = false;
        for (index, _) in text.match_indices('\n') {
            if let Some(suppression) = self.line_suppressions.last_mut() {
                if suppression.line_index == self.line_index {
                    let index = TextSize::try_from(index).expect("integer overflow");
                    let range = TextRange::at(range.start(), index);
                    suppression.text_range = suppression.text_range.cover(range);
                    did_match = true;
                }
            }

            self.line_index += 1;
        }

        if !did_match {
            if let Some(suppression) = self.line_suppressions.last_mut() {
                if suppression.line_index == self.line_index {
                    suppression.text_range = suppression.text_range.cover(range);
                }
            }
        }
    }
}

/// Signature for a suppression comment parser function
type SuppressionParser = fn(&str) -> Vec<(&str, Option<&str>)>;

type SignalHandler<'a, L, Break> = &'a mut dyn FnMut(&dyn AnalyzerSignal<L>) -> ControlFlow<Break>;

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
