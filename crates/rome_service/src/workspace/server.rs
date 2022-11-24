use super::{
    ChangeFileParams, CloseFileParams, FeatureName, FixFileResult, FormatFileParams,
    FormatOnTypeParams, FormatRangeParams, GetControlFlowGraphParams, GetFormatterIRParams,
    GetSyntaxTreeParams, GetSyntaxTreeResult, OpenFileParams, PullActionsParams, PullActionsResult,
    PullDiagnosticsParams, PullDiagnosticsResult, RenameResult, SupportsFeatureParams,
    UpdateSettingsParams,
};
use crate::file_handlers::{Capabilities, FixAllParams, Language, LintParams};
use crate::workspace::{RageEntry, RageParams, RageResult, ServerInfo, SupportsFeatureResult};
use crate::{
    file_handlers::Features,
    settings::{SettingsHandle, WorkspaceSettings},
    RomeError, Rules, Workspace,
};
use dashmap::{mapref::entry::Entry, DashMap};
use indexmap::IndexSet;
use rome_analyze::{AnalysisFilter, RuleFilter};
use rome_diagnostics::{serde::Diagnostic, DiagnosticExt};
use rome_formatter::Printed;
use rome_fs::RomePath;
use rome_js_parser::ParseDiagnostic;
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
    pub(crate) language_hint: Language,
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
    pub(crate) diagnostics: Vec<ParseDiagnostic>,
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

    /// This function transforms diagnostics coming from the parser into serializable diagnostics
    pub(crate) fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics.into_iter().map(Diagnostic::new).collect()
    }

    fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|diag| diag.is_error())
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

    /// Get the supported capabilities for a given file path
    fn get_capabilities(&self, path: &RomePath) -> Capabilities {
        let language = self
            .documents
            .get(path)
            .map(|doc| doc.language_hint)
            .unwrap_or_default();

        self.features.get_capabilities(path, language)
    }

    /// Return an error factory function for unsupported features at a given path
    fn build_capability_error<'a>(&'a self, path: &'a RomePath) -> impl FnOnce() -> RomeError + 'a {
        move || {
            let language_hint = self
                .documents
                .get(path)
                .map(|doc| doc.language_hint)
                .unwrap_or_default();

            let language = Features::get_language(path).or(language_hint);
            RomeError::SourceFileNotSupported(language, path.clone())
        }
    }

    fn build_rule_filter_list<'a>(&'a self, rules: Option<&'a Rules>) -> Vec<RuleFilter> {
        if let Some(rules) = rules {
            let enabled: IndexSet<RuleFilter> = rules.as_enabled_rules();
            enabled.into_iter().collect::<Vec<RuleFilter>>()
        } else {
            vec![]
        }
    }

    /// Get the parser result for a given file
    ///
    /// Returns and error if no file exists in the workspace with this path or
    /// if the language associated with the file has no parser capability
    fn get_parse(
        &self,
        rome_path: RomePath,
        feature: Option<FeatureName>,
    ) -> Result<AnyParse, RomeError> {
        let ignored = if let Some(feature) = feature {
            self.is_file_ignored(&rome_path, &feature)
        } else {
            false
        };

        if ignored {
            return Err(RomeError::FileIgnored(rome_path.to_path_buf()));
        }

        match self.syntax.entry(rome_path) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let rome_path = entry.key();
                let document = self.documents.get(rome_path).ok_or(RomeError::NotFound)?;

                let capabilities = self.get_capabilities(rome_path);
                let parse = capabilities
                    .parser
                    .parse
                    .ok_or_else(self.build_capability_error(rome_path))?;

                let size_limit = {
                    let settings = self.settings();
                    let settings = settings.as_ref();
                    let limit = settings.files.max_size.get();
                    usize::try_from(limit).unwrap_or(usize::MAX)
                };

                let size = document.content.as_bytes().len();
                if size >= size_limit {
                    return Err(RomeError::FileTooLarge {
                        path: rome_path.to_path_buf(),
                        size,
                        limit: size_limit,
                    });
                }

                let parsed = parse(rome_path, document.language_hint, &document.content);

                Ok(entry.insert(parsed).clone())
            }
        }
    }

    /// Takes as input the path of the file that workspace is currently processing and
    /// a list of paths to match against.
    ///
    /// If the file path matches, than `true` is returned and it should be considered ignored.
    fn is_file_ignored(&self, rome_path: &RomePath, feature: &FeatureName) -> bool {
        let settings = self.settings();
        let is_ignored_by_file_config = settings
            .as_ref()
            .files
            .ignored_files
            .matches_path(rome_path.as_path());
        match feature {
            FeatureName::Format => {
                settings
                    .as_ref()
                    .formatter
                    .ignored_files
                    .matches_path(rome_path.as_path())
                    || is_ignored_by_file_config
            }
            FeatureName::Lint => {
                settings
                    .as_ref()
                    .linter
                    .ignored_files
                    .matches_path(rome_path.as_path())
                    || is_ignored_by_file_config
            }
        }
    }
}

impl Workspace for WorkspaceServer {
    fn supports_feature(
        &self,
        params: SupportsFeatureParams,
    ) -> Result<SupportsFeatureResult, RomeError> {
        let capabilities = self.get_capabilities(&params.path);
        let settings = self.settings.read().unwrap();
        let is_ignored = self.is_file_ignored(&params.path, &params.feature);
        let result = match params.feature {
            FeatureName::Format => {
                if is_ignored {
                    SupportsFeatureResult::ignored()
                } else if capabilities.formatter.format.is_none() {
                    SupportsFeatureResult::file_not_supported()
                } else if !settings.formatter().enabled {
                    SupportsFeatureResult::disabled()
                } else {
                    SupportsFeatureResult { reason: None }
                }
            }
            FeatureName::Lint => {
                if is_ignored {
                    SupportsFeatureResult::ignored()
                } else if capabilities.analyzer.lint.is_none() {
                    SupportsFeatureResult::file_not_supported()
                } else if !settings.linter().enabled {
                    SupportsFeatureResult::disabled()
                } else {
                    SupportsFeatureResult { reason: None }
                }
            }
        };
        Ok(result)
    }

    /// Update the global settings for this workspace
    ///
    /// ## Panics
    /// This function may panic if the internal settings mutex has been poisoned
    /// by another thread having previously panicked while holding the lock
    #[tracing::instrument(level = "debug", skip(self))]
    fn update_settings(&self, params: UpdateSettingsParams) -> Result<(), RomeError> {
        let mut settings = self.settings.write().unwrap();
        settings.merge_with_configuration(params.configuration)?;
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
                language_hint: params.language_hint,
            },
        );
        Ok(())
    }

    fn get_syntax_tree(
        &self,
        params: GetSyntaxTreeParams,
    ) -> Result<GetSyntaxTreeResult, RomeError> {
        let capabilities = self.get_capabilities(&params.path);
        let debug_syntax_tree = capabilities
            .debug
            .debug_syntax_tree
            .ok_or_else(self.build_capability_error(&params.path))?;

        // The feature name here can be any feature, in theory
        let parse = self.get_parse(params.path.clone(), None)?;
        let printed = debug_syntax_tree(&params.path, parse);

        Ok(printed)
    }

    fn get_control_flow_graph(
        &self,
        params: GetControlFlowGraphParams,
    ) -> Result<String, RomeError> {
        let capabilities = self.get_capabilities(&params.path);
        let debug_control_flow = capabilities
            .debug
            .debug_control_flow
            .ok_or_else(self.build_capability_error(&params.path))?;

        let parse = self.get_parse(params.path.clone(), None)?;
        let printed = debug_control_flow(&params.path, parse, params.cursor);

        Ok(printed)
    }

    fn get_formatter_ir(&self, params: GetFormatterIRParams) -> Result<String, RomeError> {
        let capabilities = self.get_capabilities(&params.path);
        let debug_formatter_ir = capabilities
            .debug
            .debug_formatter_ir
            .ok_or_else(self.build_capability_error(&params.path))?;
        let settings = self.settings();
        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Format))?;

        if !settings.as_ref().formatter().format_with_errors && parse.has_errors() {
            return Err(RomeError::FormatWithErrorsDisabled);
        }

        debug_formatter_ir(&params.path, parse, settings)
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
        let capabilities = self.get_capabilities(&params.path);
        let lint = capabilities
            .analyzer
            .lint
            .ok_or_else(self.build_capability_error(&params.path))?;

        let settings = self.settings.read().unwrap();
        let feature = if params.categories.is_syntax() {
            FeatureName::Format
        } else {
            FeatureName::Lint
        };
        let parse = self.get_parse(params.path.clone(), Some(feature))?;
        let rules = settings.linter().rules.as_ref();
        let rule_filter_list = self.build_rule_filter_list(rules);
        let mut filter = AnalysisFilter::from_enabled_rules(Some(rule_filter_list.as_slice()));
        filter.categories = params.categories;

        let results = lint(LintParams {
            rome_path: &params.path,
            parse,
            filter,
            rules,
            settings: self.settings(),
            max_diagnostics: params.max_diagnostics,
        });

        Ok(PullDiagnosticsResult {
            diagnostics: results
                .diagnostics
                .into_iter()
                .map(|diag| {
                    let diag = diag.with_file_path(params.path.as_path().display().to_string());
                    Diagnostic::new(diag)
                })
                .collect(),
            errors: results.errors,
            skipped_diagnostics: results.skipped_diagnostics,
        })
    }

    /// Retrieves the list of code actions available for a given cursor
    /// position within a file
    fn pull_actions(&self, params: PullActionsParams) -> Result<PullActionsResult, RomeError> {
        let capabilities = self.get_capabilities(&params.path);
        let code_actions = capabilities
            .analyzer
            .code_actions
            .ok_or_else(self.build_capability_error(&params.path))?;

        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Lint))?;
        let settings = self.settings.read().unwrap();
        let rules = settings.linter().rules.as_ref();
        Ok(code_actions(
            &params.path,
            parse,
            params.range,
            rules,
            self.settings(),
        ))
    }

    /// Runs the given file through the formatter using the provided options
    /// and returns the resulting source code
    fn format_file(&self, params: FormatFileParams) -> Result<Printed, RomeError> {
        let capabilities = self.get_capabilities(&params.path);
        let format = capabilities
            .formatter
            .format
            .ok_or_else(self.build_capability_error(&params.path))?;
        let settings = self.settings();
        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Format))?;

        if !settings.as_ref().formatter().format_with_errors && parse.has_errors() {
            return Err(RomeError::FormatWithErrorsDisabled);
        }

        format(&params.path, parse, settings)
    }

    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, RomeError> {
        let capabilities = self.get_capabilities(&params.path);
        let format_range = capabilities
            .formatter
            .format_range
            .ok_or_else(self.build_capability_error(&params.path))?;
        let settings = self.settings();
        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Format))?;

        if !settings.as_ref().formatter().format_with_errors && parse.has_errors() {
            return Err(RomeError::FormatWithErrorsDisabled);
        }

        format_range(&params.path, parse, settings, params.range)
    }

    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, RomeError> {
        let capabilities = self.get_capabilities(&params.path);
        let format_on_type = capabilities
            .formatter
            .format_on_type
            .ok_or_else(self.build_capability_error(&params.path))?;

        let settings = self.settings();
        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Format))?;
        if !settings.as_ref().formatter().format_with_errors && parse.has_errors() {
            return Err(RomeError::FormatWithErrorsDisabled);
        }

        format_on_type(&params.path, parse, settings, params.offset)
    }

    fn fix_file(&self, params: super::FixFileParams) -> Result<FixFileResult, RomeError> {
        let capabilities = self.get_capabilities(&params.path);
        let fix_all = capabilities
            .analyzer
            .fix_all
            .ok_or_else(self.build_capability_error(&params.path))?;
        let settings = self.settings.read().unwrap();
        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Lint))?;

        let rules = settings.linter().rules.as_ref();
        fix_all(FixAllParams {
            rome_path: &params.path,
            parse,
            rules,
            fix_file_mode: params.fix_file_mode,
            settings: self.settings(),
        })
    }

    fn rename(&self, params: super::RenameParams) -> Result<RenameResult, RomeError> {
        let capabilities = self.get_capabilities(&params.path);
        let rename = capabilities
            .analyzer
            .rename
            .ok_or_else(self.build_capability_error(&params.path))?;

        let parse = self.get_parse(params.path.clone(), None)?;
        let result = rename(&params.path, parse, params.symbol_at, params.new_name)?;

        Ok(result)
    }

    fn rage(&self, _: RageParams) -> Result<RageResult, RomeError> {
        let entries = vec![
            RageEntry::section("Workspace"),
            RageEntry::pair("Open Documents", &format!("{}", self.documents.len())),
        ];

        Ok(RageResult { entries })
    }

    fn server_info(&self) -> Option<&ServerInfo> {
        None
    }
}
