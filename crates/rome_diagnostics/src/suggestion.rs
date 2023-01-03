use ::serde::{Deserialize, Serialize};
use rome_console::MarkupBuf;
use rome_rowan::TextRange;
use rome_text_edit::TextEdit;

/// A range that is indexed in a specific file.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FileSpan {
    pub range: TextRange,
}

/// Indicates how a tool should manage this suggestion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Applicability {
    /// The suggestion is definitely what the user intended.
    /// This suggestion should be automatically applied.
    Always,
    /// The suggestion may be what the user intended, but it is uncertain.
    /// The suggestion should result in valid JavaScript/TypeScript code if it is applied.
    MaybeIncorrect,
}

/// A Suggestion that is provided by Rome's linter, and
/// can be reported to the user, and can be automatically
/// applied if it has the right [`Applicability`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CodeSuggestion {
    pub span: FileSpan,
    pub applicability: Applicability,
    pub msg: MarkupBuf,
    pub suggestion: TextEdit,
    pub labels: Vec<TextRange>,
}
