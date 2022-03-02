use anyhow::bail;
use rome_path::RomePath;
use std::sync::Arc;

/// Internal representation of supported [language identifiers]
///
/// [language identifiers]: https://code.visualstudio.com/docs/languages/identifiers#_known-language-identifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    pub path: RomePath,
    pub language_id: Language,
    pub version: i32,
    pub text: Arc<str>,
}

impl Document {
    pub fn new(
        path: RomePath,
        language_id: Language,
        version: i32,
        text: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            path,
            language_id,
            version,
            text: text.into(),
        }
    }

    /// Retrieves the unique ID associated to the current document (file)
    pub fn file_id(&self) -> usize {
        self.path.file_id().unwrap_or(0_usize)
    }
}
