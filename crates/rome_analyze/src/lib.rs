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
pub use crate::registry::{LanguageRoot, Phase, Phases, RuleMetadata, RuleRegistry};
pub use crate::rule::{GroupLanguage, Rule, RuleAction, RuleDiagnostic, RuleGroup, RuleMeta};
pub use crate::services::{ServiceBag, ServiceBagData};
use crate::signals::DiagnosticSignal;
pub use crate::signals::{AnalyzerAction, AnalyzerSignal};
pub use crate::syntax::SyntaxVisitor;
pub use crate::visitor::{NodeVisitor, Visitor, VisitorContext};
use rome_console::markup;
use rome_diagnostics::file::FileId;
use rome_diagnostics::Diagnostic;
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
    suppressed_rules: Vec<RuleFilter<'static>>,
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
                    let result = self.flush_matches(ctx.file_id, token);
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
    /// handling line breaks, then flush all pending query signals in the queue
    /// whose position is less then the end of the token within the file
    fn flush_matches(&mut self, file_id: FileId, token: SyntaxToken<L>) -> ControlFlow<Break> {
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
                self.handle_comment(file_id, comment.text(), piece.text_range())?;
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
                self.handle_comment(file_id, comment.text(), piece.text_range())?;
            }
        }

        // Flush signals from the queue until the end of the current token is reached
        let cutoff = token.text_range().end();
        while let Some(entry) = self.signal_queue.peek() {
            let start = entry.text_range.start();
            if start >= cutoff {
                break;
            }

            // Search for an active suppression comment covering the range of
            // this signal: first try to load the last line suppression and see
            // if it matchs the current line index, otherwise perform a binary
            // search over all the previously seen suppressions to find one
            // with a matching range
            let suppression = self
                .line_suppressions
                .last()
                .filter(|suppression| {
                    suppression.line_index == self.line_index
                        && suppression.text_range.start() <= start
                })
                .or_else(|| {
                    let index = self.line_suppressions.binary_search_by(|suppression| {
                        if suppression.text_range.end() < entry.text_range.start() {
                            Ordering::Less
                        } else if entry.text_range.end() < suppression.text_range.start() {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    });

                    Some(&self.line_suppressions[index.ok()?])
                });

            let is_suppressed = suppression.map_or(false, |suppression| {
                if suppression.suppress_all {
                    return true;
                }
                suppression
                    .suppressed_rules
                    .iter()
                    .any(|filter| *filter == entry.rule)
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

    /// Parse the text content of a comment trivia piece for suppression
    /// comments, and create line suppression entries accordingly
    fn handle_comment(
        &mut self,
        file_id: FileId,
        text: &str,
        range: TextRange,
    ) -> ControlFlow<Break> {
        let mut suppress_all = false;
        let mut suppressions = Vec::new();

        for rule in (self.parse_suppression_comment)(text) {
            if let Some(rule) = rule {
                let group_rule = rule.find('/').map(|index| {
                    let (start, end) = rule.split_at(index);
                    (start, &end[1..])
                });

                let key = match group_rule {
                    None => self.query_matcher.find_group(rule).map(RuleFilter::from),
                    Some((group, rule)) => self
                        .query_matcher
                        .find_rule(group, rule)
                        .map(RuleFilter::from),
                };

                if let Some(key) = key {
                    suppressions.push(key);
                } else {
                    // Emit a warning for the unknown rule
                    let signal = DiagnosticSignal::new(move || {
                        let diag = match group_rule {
                            Some((group, rule)) => Diagnostic::warning(
                                file_id,
                                "Linter",
                                markup! {
                                    "Unknown lint rule "{group}"/"{rule}" in suppression comment"
                                },
                            ),
                            None => Diagnostic::warning(
                                file_id,
                                "Linter",
                                markup! {
                                    "Unknown lint rule group "{rule}" in suppression comment"
                                },
                            ),
                        };

                        diag.primary(range, "")
                    });

                    (self.emit_signal)(&signal)?;
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
            return ControlFlow::Continue(());
        }

        // Suppression comments apply to the next line
        let line_index = self.line_index + 1;

        // If the last suppression was on the same or previous line, extend its
        // range and set of supressed rules with the content for the new suppression
        if let Some(last_suppression) = self.line_suppressions.last_mut() {
            if last_suppression.line_index == line_index
                || last_suppression.line_index + 1 == line_index
            {
                last_suppression.line_index = line_index;
                last_suppression.text_range = last_suppression.text_range.cover(range);
                last_suppression.suppress_all |= suppress_all;
                if !last_suppression.suppress_all {
                    last_suppression.suppressed_rules.extend(suppressions);
                } else {
                    last_suppression.suppressed_rules.clear();
                }
                return ControlFlow::Continue(());
            }
        }

        let entry = LineSuppression {
            line_index,
            text_range: range,
            suppress_all,
            suppressed_rules: suppressions,
        };

        self.line_suppressions.push(entry);

        ControlFlow::Continue(())
    }

    /// Check a piece of source text (token or trivia) for line breaks and
    /// increment the line index accordingly, extending the range of the
    /// current suppression as required
    fn bump_line_index(&mut self, text: &str, range: TextRange) {
        let mut did_match = false;
        for (index, _) in text.match_indices('\n') {
            if let Some(last_suppression) = self.line_suppressions.last_mut() {
                if last_suppression.line_index == self.line_index {
                    let index = TextSize::try_from(index).expect(
                        "integer overflow while converting a suppression line to `TextSize`",
                    );
                    let range = TextRange::at(range.start(), index);
                    last_suppression.text_range = last_suppression.text_range.cover(range);
                    did_match = true;
                }
            }

            self.line_index += 1;
        }

        if !did_match {
            if let Some(last_suppression) = self.line_suppressions.last_mut() {
                if last_suppression.line_index == self.line_index {
                    last_suppression.text_range = last_suppression.text_range.cover(range);
                }
            }
        }
    }
}

/// Signature for a suppression comment parser function
///
/// This function receives the text content of a comment and returns a list of
/// lint suppressions as an optional lint rule (if the lint rule is `None` the
/// comment is interpreted as suppressing all lints)
///
/// # Examples
///
/// - `// rome-ignore format` -> `vec![]`
/// - `// rome-ignore lint` -> `vec![None]`
/// - `// rome-ignore lint(js/useWhile)` -> `vec![Some("js/useWhile")]`
/// - `// rome-ignore lint(js/useWhile) lint(js/noDeadCode)` -> `vec![Some("js/useWhile"), Some("js/noDeadCode")]`
type SuppressionParser = fn(&str) -> Vec<Option<&str>>;

type SignalHandler<'a, L, Break> = &'a mut dyn FnMut(&dyn AnalyzerSignal<L>) -> ControlFlow<Break>;

/// Allow filtering a single rule or group of rules by their names
#[derive(Debug, Clone, Copy)]
pub enum RuleFilter<'a> {
    Group(&'a str),
    Rule(&'a str, &'a str),
}

impl RuleFilter<'_> {
    /// Return `true` if the rule `R` matches this filter
    pub fn match_rule<G, R>(self) -> bool
    where
        G: RuleGroup,
        R: Rule,
    {
        match self {
            RuleFilter::Group(group) => group == G::NAME,
            RuleFilter::Rule(group, rule) => group == G::NAME && rule == R::NAME,
        }
    }
}

/// Allows filtering the list of rules that will be executed in a run of the analyzer,
/// and at what source code range signals (diagnostics or actions) may be raised
#[derive(Debug, Default, Clone, Copy)]
pub struct AnalysisFilter<'a> {
    /// Only allow rules with these categories to emit signals
    pub categories: RuleCategories,
    /// Only allow rules matching these names to emit signals
    pub enabled_rules: Option<&'a [RuleFilter<'a>]>,
    /// Do not allow rules matching these names to emit signals
    pub disabled_rules: Option<&'a [RuleFilter<'a>]>,
    /// Only emit signals matching this text range
    pub range: Option<TextRange>,
}

impl AnalysisFilter<'_> {
    /// Return `true` if the rule `R` matches this filter
    pub fn match_rule<G, R>(&self) -> bool
    where
        G: RuleGroup,
        R: Rule,
    {
        self.categories.contains(R::CATEGORY.into())
            && self.enabled_rules.map_or(true, |enabled_rules| {
                enabled_rules
                    .iter()
                    .any(|filter| filter.match_rule::<G, R>())
            })
            && self.disabled_rules.map_or(true, |disabled_rules| {
                !disabled_rules
                    .iter()
                    .any(|filter| filter.match_rule::<G, R>())
            })
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
