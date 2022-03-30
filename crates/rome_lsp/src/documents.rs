use anyhow::bail;
use rome_fs::RomePath;
use rome_js_parser::SourceType;
use std::sync::Arc;

/// Internal representation of supported [language identifiers]
///
/// [language identifiers]: https://code.visualstudio.com/docs/languages/identifiers#_known-language-identifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EditorLanguage {
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

impl From<EditorLanguage> for SourceType {
    fn from(l: EditorLanguage) -> Self {
        match l {
            EditorLanguage::JavaScript => SourceType::js_module(),
            EditorLanguage::JavaScriptReact => SourceType::jsx(),
            EditorLanguage::TypeScript => SourceType::ts(),
            EditorLanguage::TypeScriptReact => SourceType::tsx(),
        }
    }
}

/// Represents an open [`textDocument`]. Can be cheaply cloned.
///
/// [`textDocument`]: https://microsoft.github.io/language-server-protocol/specifications/specification-3-17/#textDocumentItem
#[derive(Debug, Clone)]
pub struct Document {
    pub path: RomePath,
    pub editor_language: EditorLanguage,
    pub version: i32,
    pub text: Arc<str>,
}

impl Document {
    pub fn new(
        path: RomePath,
        language_id: EditorLanguage,
        version: i32,
        text: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            path,
            editor_language: language_id,
            version,
            text: text.into(),
        }
    }

    /// Retrieves the unique ID associated to the current document (file)
    pub fn file_id(&self) -> usize {
        self.path.file_id().unwrap_or(0_usize)
    }

    pub fn get_source_type(&self) -> SourceType {
        self.editor_language.into()
    }
}
