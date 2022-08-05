use super::{
    ChangeFileParams, CloseFileParams, FeatureName, FixFileResult, FormatFileParams,
    FormatOnTypeParams, FormatRangeParams, GetControlFlowGraphParams, GetFormatterIRParams,
    GetSyntaxTreeParams, GetSyntaxTreeResult, OpenFileParams, PullActionsParams, PullActionsResult,
    PullDiagnosticsParams, PullDiagnosticsResult, RenameResult, SupportsFeatureParams,
    UpdateSettingsParams,
};
use crate::file_handlers::FixAllParams;
use crate::{
    file_handlers::Features,
    settings::{SettingsHandle, WorkspaceSettings},
    RomeError, Workspace,
};
use dashmap::{mapref::entry::Entry, DashMap};
use indexmap::IndexSet;
use rome_analyze::{AnalysisFilter, RuleFilter};
use rome_diagnostics::{Diagnostic, Severity};
use rome_formatter::Printed;
use rome_fs::RomePath;
use rome_rowan::{AstNode, Language as RowanLanguage, SendNode, SyntaxNode};
use std::{any::type_name, panic::RefUnwindSafe, sync::RwLock};

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

    fn settings(&self) -> SettingsHandle {
        SettingsHandle::new(&self.settings)
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

    fn get_syntax_tree(
        &self,
        params: GetSyntaxTreeParams,
    ) -> Result<GetSyntaxTreeResult, RomeError> {
        let capabilities = self.features.get_capabilities(&params.path);
        let printer = capabilities
            .debug_syntax_tree
            .ok_or_else(|| RomeError::SourceFileNotSupported(params.path.clone()))?;

        let parse = self.get_parse(params.path.clone())?;
        let printed = printer(&params.path, parse);

        Ok(printed)
    }

    fn get_control_flow_graph(
        &self,
        params: GetControlFlowGraphParams,
    ) -> Result<String, RomeError> {
        let capabilities = self.features.get_capabilities(&params.path);
        let printer = capabilities
            .debug_control_flow
            .ok_or_else(|| RomeError::SourceFileNotSupported(params.path.clone()))?;

        let parse = self.get_parse(params.path.clone())?;
        let printed = printer(&params.path, parse, params.cursor);

        Ok(printed)
    }

    fn get_formatter_ir(&self, params: GetFormatterIRParams) -> Result<String, RomeError> {
        let capabilities = self.features.get_capabilities(&params.path);
        let printer = capabilities
            .debug_formatter_ir
            .ok_or_else(|| RomeError::SourceFileNotSupported(params.path.clone()))?;

        let parse = self.get_parse(params.path.clone())?;
        let settings = self.settings();

        if !settings.as_ref().format.format_with_errors && parse.has_errors() {
            return Err(RomeError::FormatWithErrorsDisabled);
        }

        printer(&params.path, parse, settings)
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
    ) -> Result<PullDiagnosticsResult, RomeError> {
        let capabilities = self.features.get_capabilities(&params.path);
        let linter = capabilities
            .lint
            .ok_or_else(|| RomeError::SourceFileNotSupported(params.path.clone()))?;

        let parse = self.get_parse(params.path.clone())?;
        let settings = self.settings.read().unwrap();
        let rules = settings.linter.rules.as_ref();
        let enabled_rules: Option<Vec<RuleFilter>> = if let Some(rules) = rules {
            let enabled: IndexSet<RuleFilter> = rules.as_enabled_rules();
            Some(enabled.into_iter().collect())
        } else {
            None
        };

        let mut filter = match &enabled_rules {
            Some(rules) => AnalysisFilter::from_enabled_rules(Some(rules.as_slice())),
            _ => AnalysisFilter::default(),
        };

        filter.categories = params.categories;
        let diagnostics = linter(&params.path, parse, filter, rules);

        Ok(PullDiagnosticsResult { diagnostics })
    }

    /// Retrieves the list of code actions available for a given cursor
    /// position within a file
    fn pull_actions(&self, params: PullActionsParams) -> Result<PullActionsResult, RomeError> {
        let capabilities = self.features.get_capabilities(&params.path);
        let code_actions = capabilities
            .code_actions
            .ok_or_else(|| RomeError::SourceFileNotSupported(params.path.clone()))?;

        let parse = self.get_parse(params.path.clone())?;

        let settings = self.settings.read().unwrap();
        let rules = settings.linter.rules.as_ref();

        Ok(code_actions(&params.path, parse, params.range, rules))
    }

    /// Runs the given file through the formatter using the provided options
    /// and returns the resulting source code
    fn format_file(&self, params: FormatFileParams) -> Result<Printed, RomeError> {
        let capabilities = self.features.get_capabilities(&params.path);
        let formatter = capabilities
            .format
            .ok_or_else(|| RomeError::SourceFileNotSupported(params.path.clone()))?;

        let parse = self.get_parse(params.path.clone())?;
        let settings = self.settings();

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
        let settings = self.settings();

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
        let settings = self.settings();

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
        let settings = self.settings.read().unwrap();
        let rules = settings.linter.rules.as_ref();
        fix_all(FixAllParams {
            rome_path: &params.path,
            parse,
            rules,
            fix_file_mode: params.fix_file_mode,
        })
    }

    fn rename(&self, params: super::RenameParams) -> Result<RenameResult, RomeError> {
        let capabilities = self.features.get_capabilities(&params.path);
        let rename = capabilities
            .rename
            .ok_or_else(|| RomeError::SourceFileNotSupported(params.path.clone()))?;

        let parse = self.get_parse(params.path.clone())?;
        let result = rename(&params.path, parse, params.symbol_at, params.new_name)?;

        Ok(result)
    }
}
