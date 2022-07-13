use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    hash::{Hash, Hasher},
    ptr,
};

use rome_diagnostics::file::FileId;
use rome_rowan::{Language, TextRange};

use crate::{AnalyzerSignal, Phases, QueryMatch, ServiceBag};

/// The [QueryMatcher] trait is responsible of running lint rules on
/// [QueryMatch] instances emitted by the various [Visitor](crate::Visitor)
/// and push signals wrapped in [SignalEntry] to the signal queue
pub trait QueryMatcher<L: Language> {
    /// Return a unique identifier for a rule if it's known by this query matcher
    fn find_rule(&self, name: &str) -> Option<RuleKey>;

    /// Execute a single query match
    fn match_query(&mut self, params: MatchQueryParams<L>);
}

/// Parameters provided to [QueryMatcher::match_query] and require to run lint rules
pub struct MatchQueryParams<'a, L: Language> {
    pub phase: Phases,
    pub file_id: FileId,
    pub root: &'a L::Root,
    pub query: QueryMatch<L>,
    pub services: &'a ServiceBag,
    pub signal_queue: &'a mut BinaryHeap<SignalEntry<L>>,
}

/// Newtype wrapper around the name of a rule, implementing equality and
/// hashing based on the raw string pointer instead of comparing the string
/// itself
#[derive(Copy, Clone, Debug, Eq)]
pub struct RuleKey(pub &'static str);

impl PartialEq for RuleKey {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.0, other.0)
    }
}

impl Hash for RuleKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self.0, state)
    }
}

/// Entry for a pending signal in the `signal_queue`
pub struct SignalEntry<L: Language> {
    /// Boxed analyzer signal to be emitted
    pub signal: Box<dyn AnalyzerSignal<L>>,
    /// Unique identifier for the rule that emitted this signal
    pub rule: RuleKey,
    /// Text range in the document this signal covers
    pub text_range: TextRange,
}

// SignalEntry is ordered based on the starting point of its `text_range`
impl<L: Language> Ord for SignalEntry<L> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.text_range.start().cmp(&self.text_range.start())
    }
}

impl<L: Language> PartialOrd for SignalEntry<L> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<L: Language> Eq for SignalEntry<L> {}

impl<L: Language> PartialEq for SignalEntry<L> {
    fn eq(&self, other: &Self) -> bool {
        self.text_range.start() == other.text_range.start()
    }
}
