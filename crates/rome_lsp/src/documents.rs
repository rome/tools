use anyhow::bail;

use crate::line_index::LineIndex;

/// Internal representation of supported [language identifiers]
///
/// [language identifiers]: https://code.visualstudio.com/docs/languages/identifiers#_known-language-identifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum EditorLanguage {
    JavaScript,
    JavaScriptReact,
    TypeScript,
    TypeScriptReact,
}

impl TryFrom<&str> for EditorLanguage {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "javascript" => Ok(EditorLanguage::JavaScript),
            "javascriptreact" => Ok(EditorLanguage::JavaScriptReact),
            "typescript" => Ok(EditorLanguage::TypeScript),
            "typescriptreact" => Ok(EditorLanguage::TypeScriptReact),
            _ => bail!("Unsupported language: {}", value),
        }
    }
}

/// Represents an open [`textDocument`]. Can be cheaply cloned.
///
/// [`textDocument`]: https://microsoft.github.io/language-server-protocol/specifications/specification-3-17/#textDocumentItem
#[derive(Clone)]
pub(crate) struct Document {
    pub(crate) version: i32,
    pub(crate) line_index: LineIndex,
}

impl Document {
    pub(crate) fn new(version: i32, text: &str) -> Self {
        Self {
            version,
            line_index: LineIndex::new(text),
        }
    }
}
