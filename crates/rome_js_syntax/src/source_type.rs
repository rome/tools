use std::fmt::Display;
use std::path::{Path, PathBuf};

/// Enum of the different ECMAScript standard versions.
/// The versions are ordered in increasing order; The newest version comes last.
///
/// Defaults to the latest stable ECMAScript standard.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LanguageVersion {
    ES2022,

    /// The next, not yet finalized ECMAScript version
    ESNext,
}

impl LanguageVersion {
    /// Returns the latest finalized ECMAScript version
    pub const fn latest() -> Self {
        LanguageVersion::ES2022
    }
}

impl Default for LanguageVersion {
    fn default() -> Self {
        Self::latest()
    }
}

/// Is the source file an ECMAScript Module or Script.
/// Changes the parsing semantic.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum ModuleKind {
    /// An ECMAScript [Script](https://tc39.es/ecma262/multipage/ecmascript-language-scripts-and-modules.html#sec-scripts)
    Script,

    /// AN ECMAScript [Module](https://tc39.es/ecma262/multipage/ecmascript-language-scripts-and-modules.html#sec-modules)
    #[default]
    Module,
}

impl ModuleKind {
    pub const fn is_script(&self) -> bool {
        matches!(self, ModuleKind::Script)
    }
    pub const fn is_module(&self) -> bool {
        matches!(self, ModuleKind::Module)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum LanguageVariant {
    /// Standard JavaScript or TypeScript syntax without any extensions
    #[default]
    Standard,

    /// Allows JSX syntax inside a JavaScript or TypeScript file
    Jsx,
}

impl LanguageVariant {
    pub const fn is_standard(&self) -> bool {
        matches!(self, LanguageVariant::Standard)
    }
    pub const fn is_jsx(&self) -> bool {
        matches!(self, LanguageVariant::Jsx)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum Language {
    #[default]
    JavaScript,

    /// TypeScript source with or without JSX.
    /// `definition_file` must be true for `d.ts` files.
    TypeScript { definition_file: bool },
}

impl Language {
    pub const fn is_javascript(&self) -> bool {
        matches!(self, Language::JavaScript)
    }
    pub const fn is_typescript(&self) -> bool {
        matches!(self, Language::TypeScript { .. })
    }

    pub const fn is_definition_file(&self) -> bool {
        matches!(
            self,
            Language::TypeScript {
                definition_file: true
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SourceType {
    language: Language,
    variant: LanguageVariant,
    module_kind: ModuleKind,
    version: LanguageVersion,
}

impl SourceType {
    /// language: JS, variant: Standard, module_kind: Module, version: Latest
    pub fn js_module() -> Self {
        Self::default()
    }

    /// language: JS, variant: Standard, module_kind: Script, version: Latest
    pub fn js_script() -> Self {
        Self::default().with_module_kind(ModuleKind::Script)
    }

    /// language: JS, variant: JSX, module_kind: Module, version: Latest
    pub fn jsx() -> SourceType {
        Self::js_module().with_variant(LanguageVariant::Jsx)
    }

    /// language: TS, variant: Standard, module_kind: Module, version: Latest
    pub fn ts() -> SourceType {
        Self {
            language: Language::TypeScript {
                definition_file: false,
            },
            ..Self::default()
        }
    }

    /// language: TS, variant: JSX, module_kind: Module, version: Latest
    pub fn tsx() -> SourceType {
        Self::ts().with_variant(LanguageVariant::Jsx)
    }

    /// TypeScript definition file
    /// language: TS, ambient, variant: Standard, module_kind: Module, version: Latest
    pub fn d_ts() -> SourceType {
        Self {
            language: Language::TypeScript {
                definition_file: true,
            },
            ..Self::default()
        }
    }

    pub const fn with_module_kind(mut self, kind: ModuleKind) -> Self {
        self.module_kind = kind;
        self
    }

    pub const fn with_version(mut self, version: LanguageVersion) -> Self {
        self.version = version;
        self
    }

    pub const fn with_variant(mut self, variant: LanguageVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn variant(&self) -> LanguageVariant {
        self.variant
    }

    pub fn version(&self) -> LanguageVersion {
        self.version
    }

    pub const fn module_kind(&self) -> ModuleKind {
        self.module_kind
    }

    pub const fn is_module(&self) -> bool {
        self.module_kind.is_module()
    }
}

impl TryFrom<&Path> for SourceType {
    type Error = SourceTypeError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file_name = path
            .file_name()
            .ok_or_else(|| SourceTypeError::MissingFileName(path.into()))?
            .to_str()
            .ok_or_else(|| SourceTypeError::MissingFileName(path.into()))?;

        let extension = path
            .extension()
            .ok_or_else(|| SourceTypeError::MissingFileExtension(path.into()))?
            .to_str()
            .ok_or_else(|| SourceTypeError::MissingFileExtension(path.into()))?;

        compute_source_type_from_path_or_extension(file_name, extension)
    }
}

/// Errors around the construct of the source type
#[derive(Debug)]
pub enum SourceTypeError {
    /// The path has no file name
    MissingFileName(PathBuf),
    /// The path has no file extension
    MissingFileExtension(PathBuf),
    /// The source type is unknown
    UnknownExtension(String),
}

impl std::error::Error for SourceTypeError {}

impl Display for SourceTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceTypeError::MissingFileName(path) => {
                write!(f, "The path {path:?} has no file name")
            }
            SourceTypeError::MissingFileExtension(path) => {
                write!(f, "The path {path:?} has no file extension")
            }
            SourceTypeError::UnknownExtension(extension) => {
                write!(f, "The parser can't parse the extension '{extension}' yet")
            }
        }
    }
}

/// It deduce the [SourceType] from the file name and its extension
fn compute_source_type_from_path_or_extension(
    file_name: &str,
    extension: &str,
) -> Result<SourceType, SourceTypeError> {
    let source_type = if file_name.ends_with(".d.ts") || file_name.ends_with(".d.mts") {
        SourceType::d_ts()
    } else if file_name.ends_with(".d.cts") {
        SourceType::d_ts().with_module_kind(ModuleKind::Script)
    } else {
        match extension {
            "cjs" => SourceType::js_module().with_module_kind(ModuleKind::Script),
            "js" | "mjs" | "jsx" => SourceType::jsx(),
            "ts" | "mts" => SourceType::ts(),
            "cts" => SourceType::ts().with_module_kind(ModuleKind::Script),
            "tsx" => SourceType::tsx(),
            _ => return Err(SourceTypeError::UnknownExtension(extension.into())),
        }
    };
    Ok(source_type)
}
