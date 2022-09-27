use crate::{file::FileSpan, *};
use rome_console::MarkupBuf;
use rome_rowan::TextRange;
use rome_text_edit::TextEdit;
use serde::{Deserialize, Serialize};

/// A Suggestion that is provided by rslint, and
/// can be reported to the user, and can be automatically
/// applied if it has the right [`Applicability`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CodeSuggestion {
    /// If the `FileId` is `None`, it's in the same file as
    /// his parent.
    pub span: FileSpan,
    pub applicability: Applicability,
    pub msg: MarkupBuf,
    pub suggestion: TextEdit,
    pub labels: Vec<TextRange>,
}
