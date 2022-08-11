use crate::{file::FileSpan, *};
use rome_console::MarkupBuf;
use rome_rowan::TextRange;
use rome_text_edit::Indel;

/// A Suggestion that is provided by rslint, and
/// can be reported to the user, and can be automatically
/// applied if it has the right [`Applicability`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct CodeSuggestion {
    /// If the `FileId` is `None`, it's in the same file as
    /// his parent.
    pub substitution: SuggestionChange,
    pub span: FileSpan,
    pub applicability: Applicability,
    pub msg: MarkupBuf,
    pub style: SuggestionStyle,
    #[cfg_attr(feature = "serde", schemars(with = "Vec<rome_rowan::TextRangeSchema>"))]
    pub labels: Vec<TextRange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub enum SuggestionChange {
    Indels(Vec<Indel>),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
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
