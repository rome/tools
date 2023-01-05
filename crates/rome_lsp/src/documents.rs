use anyhow::Result;
use rome_rowan::TextRange;

use crate::line_index::LineIndex;

/// Represents an open [`textDocument`]. Can be cheaply cloned.
///
/// [`textDocument`]: https://microsoft.github.io/language-server-protocol/specifications/specification-3-17/#textDocumentItem
#[derive(Clone)]
pub(crate) struct Document {
    pub(crate) version: i32,
    pub(crate) line_index: LineIndex,
}

impl Document {
    pub(crate) fn new(version: i32, text: impl Into<String>) -> Self {
        Self {
            version,
            line_index: LineIndex::new(text),
        }
    }

    pub(crate) fn text(&self) -> &str {
        self.line_index.text()
    }

    pub(crate) fn replace_range(&mut self, range: TextRange, replace_with: &str) -> Result<()> {
        self.line_index.replace_range(range, replace_with)
    }
}
