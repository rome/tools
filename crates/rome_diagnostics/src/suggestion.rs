use crate::{file::FileSpan, *};
use rome_text_edit::Indel;
use std::ops::Range;

/// A Suggestion that is provided by rslint, and
/// can be reported to the user, and can be automatically
/// applied if it has the right [`Applicability`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodeSuggestion {
    /// If the `FileId` is `None`, it's in the same file as
    /// his parent.
    pub substitution: SuggestionChange,
    pub span: FileSpan,
    pub applicability: Applicability,
    pub msg: String,
    pub style: SuggestionStyle,
    pub labels: Vec<Range<usize>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SuggestionChange {
    Indels(Vec<Indel>),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SuggestionStyle {
    /// Do not show the suggestion at all
    DontShow,
    /// Show as inline but do not show the code
    HideCode,
    /// Show as inline, e.g. `help: do this: `code``
    Inline,
    /// Show as a separate block, e.g.
    /// ```text
    /// help: try this
    ///    |
    ///  2 | let a = 6;
    ///    |    ^
    /// ```
    Full,
}
