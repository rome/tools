#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ActionCategory {
    SafeFix,
    Suggestion,
    Refactor,
}
