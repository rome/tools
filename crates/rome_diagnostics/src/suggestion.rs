use crate::{file::FileSpan, *};
use rome_console::MarkupBuf;
use rome_rowan::TextRange;
use rome_text_edit::Indel;
use serde::{Deserialize, Serialize};

/// A Suggestion that is provided by rslint, and
/// can be reported to the user, and can be automatically
/// applied if it has the right [`Applicability`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CodeSuggestion {
    /// If the `FileId` is `None`, it's in the same file as
    /// his parent.
    pub substitution: SuggestionChange,
    pub span: FileSpan,
    pub applicability: Applicability,
    pub msg: MarkupBuf,
    pub style: SuggestionStyle,
    pub labels: Vec<TextRange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum SuggestionChange {
    Indels(Vec<Indel>),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum SuggestionStyle {
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
