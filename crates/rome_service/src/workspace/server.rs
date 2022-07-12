use std::{any::type_name, panic::RefUnwindSafe, sync::RwLock};

use dashmap::{mapref::entry::Entry, DashMap};
use rome_analyze::AnalyzerAction;
use rome_diagnostics::{Diagnostic, Severity};
use rome_formatter::Printed;
use rome_fs::RomePath;
use rome_js_syntax::JsLanguage;
use rome_rowan::{AstNode, Language as RowanLanguage, SendNode, SyntaxNode};

use crate::{
    file_handlers::Features,
    settings::{SettingsHandle, WorkspaceSettings},
    RomeError, Workspace,
};

use super::{
    ChangeFileParams, CloseFileParams, FeatureName, FixFileResult, FormatFileParams,
    FormatOnTypeParams, FormatRangeParams, GetSyntaxTreeParams, OpenFileParams, PullActionsParams,
    PullDiagnosticsParams, SupportsFeatureParams, UpdateSettingsParams,
};

pub(super) struct WorkspaceServer {
    /// features available throughout the application
    features: Features,
    /// global settings object for this workspace
    settings: RwLock<WorkspaceSettings>,
    /// Stores the document (text content + version number) associated with a URL
    documents: DashMap<RomePath, Document>,
    /// Stores the result of the parser (syntax tree + diagnostics) for a given URL
    syntax: DashMap<RomePath, AnyParse>,
}

/// The `Workspace` object is long lived, so we want it to be able to cross
/// unwind boundaries.
/// In return we have to make sure operations on the workspace either do not
/// panic, of that panicking will not result in any broken invariant (it would
/// not result in any undefined behavior as catching an unwind is safe, but it
/// could lead to hard to debug issues)
impl RefUnwindSafe for WorkspaceServer {}

#[derive(Clone, Debug)]
pub(crate) struct Document {
    pub(crate) content: String,
    pub(crate) version: i32,
}

/// Language-independent cache entry for a parsed file
///
/// This struct holds a handle to the root node of the parsed syntax tree,
/// along with the list of diagnostics emitted by the parser while generating
/// this entry.
///
/// It can be dynamically downcast into a concrete [SyntaxNode] or [AstNode] of
/// the corresponding language, generally through a language-specific capability
#[derive(Clone)]
pub(crate) struct AnyParse {
    pub(crate) root: SendNode,
    pub(crate) diagnostics: Vec<Diagnostic>,
}

impl AnyParse {
    pub(crate) fn syntax<L>(&self) -> SyntaxNode<L>
    where
        L: RowanLanguage + 'static,
    {
        self.root.clone().into_node().unwrap_or_else(|| {
            panic!(
                "could not downcast root node to language {}",
                type_name::<L>()
            )
        })
    }

    pub(crate) fn tree<N>(&self) -> N
    where
        N: AstNode,
        N::Language: 'static,
    {
        N::unwrap_cast(self.syntax::<N::Language>())
    }

    pub(crate) fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diag| diag.severity >= Severity::Error)
    }
}

impl WorkspaceServer {
    /// Create a new [Workspace]
    ///
    /// This is implemented as a crate-private method instead of using
    /// [Default] to disallow instances of [Workspace] from being created
    /// outside of a [crate::App]
    pub(crate) fn new() -> Self {
        Self {
            features: Features::new(),
            settings: RwLock::default(),
            documents: DashMap::default(),
            syntax: DashMap::default(),
        }
    }

    fn settings<E>(&self, editor: E) -> SettingsHandle<E> {
        SettingsHandle::new(&self.settings, editor)
    }

    /// Get the parser result for a given file
    ///
    /// Returns and error if no file exists in the workspace with this path or
    /// if the language associated with the file has no parser capability
    fn get_parse(&self, rome_path: RomePath) -> Result<AnyParse, RomeError> {
        match self.syntax.entry(rome_path) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let rome_path = entry.key();
                let document = self.documents.get(rome_path).ok_or(RomeError::NotFound)?;

                let capabilities = self.features.get_capabilities(rome_path);
                let parser = capabilities
                    .parse
                    .ok_or_else(|| RomeError::SourceFileNotSupported(rome_path.clone()))?;

                let parsed = parser(rome_path, &document.content);

                Ok(entry.insert(parsed).clone())
            }
        }
    }
}

impl Workspace for WorkspaceServer {
    fn supports_feature(&self, params: SupportsFeatureParams) -> bool {
        let capabilities = self.features.get_capabilities(&params.path);
        let settings = self.settings.read().unwrap();
        match params.feature {
            FeatureName::Format => capabilities.format.is_some() && settings.format.enabled,
            FeatureName::Lint => capabilities.lint.is_some() && settings.linter.enabled,
        }
    }

    /// Update the global settings for this workspace
    ///
    /// ## Panics
    /// This function may panic if the internal settings mutex has been poisoned
    /// by another thread having previously panicked while holding the lock
    fn update_settings(&self, params: UpdateSettingsParams) -> Result<(), RomeError> {
        let mut settings = self.settings.write().unwrap();
        *settings = params.settings;
        Ok(())
    }

    /// Add a new file to the workspace
    fn open_file(&self, params: OpenFileParams) -> Result<(), RomeError> {
        self.syntax.remove(&params.path);
        self.documents.insert(
            params.path,
            Document {
                content: params.content,
                version: params.version,
            },
        );
        Ok(())
    }

    fn get_syntax_tree(&self, params: GetSyntaxTreeParams) -> Result<String, RomeError> {
        let capabilities = self.features.get_capabilities(&params.path);
        let printer = capabilities
            .debug_print
            .ok_or_else(|| RomeError::SourceFileNotSupported(params.path.clone()))?;

        let parse = self.get_parse(params.path.clone())?;
        let printed = printer(&params.path, parse);

        Ok(printed)
    }

    /// Change the content of an open file
    fn change_file(&self, params: ChangeFileParams) -> Result<(), RomeError> {
        let mut document = self
            .documents
            .get_mut(&params.path)
            .ok_or(RomeError::NotFound)?;

        debug_assert!(params.version > document.version);
        document.version = params.version;
        document.content = params.content;

        self.syntax.remove(&params.path);
        Ok(())
    }

    /// Remove a file from the workspace
    fn close_file(&self, params: CloseFileParams) -> Result<(), RomeError> {
        self.documents
            .remove(&params.path)
            .ok_or(RomeError::NotFound)?;

        self.syntax.remove(&params.path);
        Ok(())
    }

    /// Retrieves the list of diagnostics associated with a file
    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<Vec<Diagnostic>, RomeError> {
        let capabilities = self.features.get_capabilities(&params.path);
        let linter = capabilities
            .lint
            .ok_or_else(|| RomeError::SourceFileNotSupported(params.path.clone()))?;

        let parse = self.get_parse(params.path.clone())?;

        Ok(linter(&params.path, parse, params.categories))
    }

    /// Retrieves the list of code actions available for a given cursor
    /// position within a file
    fn pull_actions(
        &self,
        params: PullActionsParams,
    ) -> Result<Vec<AnalyzerAction<JsLanguage>>, RomeError> {
        let capabilities = self.features.get_capabilities(&params.path);
        let code_actions = capabilities
            .code_actions
            .ok_or_else(|| RomeError::SourceFileNotSupported(params.path.clone()))?;

        let parse = self.get_parse(params.path.clone())?;

        Ok(code_actions(&params.path, parse, params.range))
    }

    /// Runs the given file through the formatter using the provided options
    /// and returns the resulting source code
    fn format_file(&self, params: FormatFileParams) -> Result<Printed, RomeError> {
        let capabilities = self.features.get_capabilities(&params.path);
        let formatter = capabilities
            .format
            .ok_or_else(|| RomeError::SourceFileNotSupported(params.path.clone()))?;

        let parse = self.get_parse(params.path.clone())?;
        let settings = self.settings(params.indent_style);

        if !settings.as_ref().format.format_with_errors && parse.has_errors() {
            return Err(RomeError::FormatWithErrorsDisabled);
        }

        formatter(&params.path, parse, settings)
    }

    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, RomeError> {
        let capabilities = self.features.get_capabilities(&params.path);
        let formatter = capabilities
            .format_range
            .ok_or_else(|| RomeError::SourceFileNotSupported(params.path.clone()))?;

        let parse = self.get_parse(params.path.clone())?;
        let settings = self.settings(params.indent_style);

        if !settings.as_ref().format.format_with_errors && parse.has_errors() {
            return Err(RomeError::FormatWithErrorsDisabled);
        }

        formatter(&params.path, parse, settings, params.range)
    }

    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, RomeError> {
        let capabilities = self.features.get_capabilities(&params.path);
        let formatter = capabilities
            .format_on_type
            .ok_or_else(|| RomeError::SourceFileNotSupported(params.path.clone()))?;

        let parse = self.get_parse(params.path.clone())?;
        let settings = self.settings(params.indent_style);

        if !settings.as_ref().format.format_with_errors && parse.has_errors() {
            return Err(RomeError::FormatWithErrorsDisabled);
        }

        formatter(&params.path, parse, settings, params.offset)
    }

    fn fix_file(&self, params: super::FixFileParams) -> Result<FixFileResult, RomeError> {
        let capabilities = self.features.get_capabilities(&params.path);
        let fix_all = capabilities
            .fix_all
            .ok_or_else(|| RomeError::SourceFileNotSupported(params.path.clone()))?;

        let parse = self.get_parse(params.path.clone())?;

        Ok(fix_all(&params.path, parse))
    }
}
