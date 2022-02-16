use std::sync::Arc;

use anyhow::bail;
use rome_analyze::FileId;

/// Internal representation of supported [language identifiers]
///
/// [language identifiers]: https://code.visualstudio.com/docs/languages/identifiers#_known-language-identifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Language {
    JavaScript,
    TypeScript,
}

impl TryFrom<&str> for Language {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "javascript" => Ok(Language::JavaScript),
            "typescript" => Ok(Language::TypeScript),
            _ => bail!("Unsupported language: {}", value),
        }
    }
}

/// Represents an open [`textDocument`]. Can be cheaply cloned.
///
/// [`textDocument`]: https://microsoft.github.io/language-server-protocol/specifications/specification-3-17/#textDocumentItem
#[derive(Clone)]
pub struct Document {
    pub file_id: FileId,
    pub language_id: Language,
    pub version: i32,
    pub text: Arc<str>,
}

impl Document {
    pub fn new(
        file_id: FileId,
        language_id: Language,
        version: i32,
        text: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            file_id,
            language_id,
            version,
            text: text.into(),
        }
    }
}
