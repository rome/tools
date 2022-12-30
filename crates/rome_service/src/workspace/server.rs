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
    Rules, Workspace, WorkspaceError,
};
use dashmap::{mapref::entry::Entry, DashMap};
use indexmap::IndexSet;
use rome_analyze::{AnalysisFilter, RuleFilter};
use rome_diagnostics::{serde::Diagnostic as SerdeDiagnostic, Diagnostic, DiagnosticExt, Severity};
use rome_formatter::Printed;
use rome_fs::RomePath;
use rome_parser::AnyParse;
use std::ffi::OsStr;
use std::{panic::RefUnwindSafe, sync::RwLock};

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
    fn build_capability_error<'a>(
        &'a self,
        path: &'a RomePath,
    ) -> impl FnOnce() -> WorkspaceError + 'a {
        move || {
            let language_hint = self
                .documents
                .get(path)
                .map(|doc| doc.language_hint)
                .unwrap_or_default();

            let language = Features::get_language(path).or(language_hint);
            WorkspaceError::source_file_not_supported(
                language,
                path.clone().display().to_string(),
                path.clone()
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(|s| s.to_string()),
            )
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
    ) -> Result<AnyParse, WorkspaceError> {
        let ignored = if let Some(feature) = feature {
            self.is_file_ignored(&rome_path, &feature)
        } else {
            false
        };

        if ignored {
            return Err(WorkspaceError::file_ignored(format!(
                "{}",
                rome_path.to_path_buf().display()
            )));
        }

        match self.syntax.entry(rome_path) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let rome_path = entry.key();
                let document = self
                    .documents
                    .get(rome_path)
                    .ok_or_else(WorkspaceError::not_found)?;

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
                    return Err(WorkspaceError::file_too_large(
                        rome_path.to_path_buf().display().to_string(),
                        size,
                        size_limit,
                    ));
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
    ) -> Result<SupportsFeatureResult, WorkspaceError> {
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
    fn update_settings(&self, params: UpdateSettingsParams) -> Result<(), WorkspaceError> {
        let mut settings = self.settings.write().unwrap();
        settings.merge_with_configuration(params.configuration)?;
        Ok(())
    }

    /// Add a new file to the workspace
    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError> {
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
    ) -> Result<GetSyntaxTreeResult, WorkspaceError> {
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
    ) -> Result<String, WorkspaceError> {
        let capabilities = self.get_capabilities(&params.path);
        let debug_control_flow = capabilities
            .debug
            .debug_control_flow
            .ok_or_else(self.build_capability_error(&params.path))?;

        let parse = self.get_parse(params.path.clone(), None)?;
        let printed = debug_control_flow(&params.path, parse, params.cursor);

        Ok(printed)
    }

    fn get_formatter_ir(&self, params: GetFormatterIRParams) -> Result<String, WorkspaceError> {
        let capabilities = self.get_capabilities(&params.path);
        let debug_formatter_ir = capabilities
            .debug
            .debug_formatter_ir
            .ok_or_else(self.build_capability_error(&params.path))?;
        let settings = self.settings();
        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Format))?;

        if !settings.as_ref().formatter().format_with_errors && parse.has_errors() {
            return Err(WorkspaceError::format_with_errors_disabled());
        }

        debug_formatter_ir(&params.path, parse, settings)
    }

    /// Change the content of an open file
    fn change_file(&self, params: ChangeFileParams) -> Result<(), WorkspaceError> {
        let mut document = self
            .documents
            .get_mut(&params.path)
            .ok_or_else(WorkspaceError::not_found)?;

        debug_assert!(params.version > document.version);
        document.version = params.version;
        document.content = params.content;

        self.syntax.remove(&params.path);
        Ok(())
    }

    /// Remove a file from the workspace
    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError> {
        self.documents
            .remove(&params.path)
            .ok_or_else(WorkspaceError::not_found)?;

        self.syntax.remove(&params.path);
        Ok(())
    }

    /// Retrieves the list of diagnostics associated with a file
    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, WorkspaceError> {
        let feature = if params.categories.is_syntax() {
            FeatureName::Format
        } else {
            FeatureName::Lint
        };

        let parse = self.get_parse(params.path.clone(), Some(feature))?;
        let settings = self.settings.read().unwrap();

        let (diagnostics, errors, skipped_diagnostics) = if let Some(lint) =
            self.get_capabilities(&params.path).analyzer.lint
        {
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

            (
                results.diagnostics,
                results.errors,
                results.skipped_diagnostics,
            )
        } else {
            let parse_diagnostics = parse.into_diagnostics();
            let errors = parse_diagnostics
                .iter()
                .filter(|diag| diag.severity() <= Severity::Error)
                .count();

            (parse_diagnostics, errors, 0)
        };

        Ok(PullDiagnosticsResult {
            diagnostics: diagnostics
                .into_iter()
                .map(|diag| {
                    let diag = diag.with_file_path(params.path.as_path().display().to_string());
                    SerdeDiagnostic::new(diag)
                })
                .collect(),
            errors,
            skipped_diagnostics,
        })
    }

    /// Retrieves the list of code actions available for a given cursor
    /// position within a file
    fn pull_actions(&self, params: PullActionsParams) -> Result<PullActionsResult, WorkspaceError> {
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
    fn format_file(&self, params: FormatFileParams) -> Result<Printed, WorkspaceError> {
        let capabilities = self.get_capabilities(&params.path);
        let format = capabilities
            .formatter
            .format
            .ok_or_else(self.build_capability_error(&params.path))?;
        let settings = self.settings();
        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Format))?;

        if !settings.as_ref().formatter().format_with_errors && parse.has_errors() {
            return Err(WorkspaceError::format_with_errors_disabled());
        }

        format(&params.path, parse, settings)
    }

    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, WorkspaceError> {
        let capabilities = self.get_capabilities(&params.path);
        let format_range = capabilities
            .formatter
            .format_range
            .ok_or_else(self.build_capability_error(&params.path))?;
        let settings = self.settings();
        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Format))?;

        if !settings.as_ref().formatter().format_with_errors && parse.has_errors() {
            return Err(WorkspaceError::format_with_errors_disabled());
        }

        format_range(&params.path, parse, settings, params.range)
    }

    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, WorkspaceError> {
        let capabilities = self.get_capabilities(&params.path);
        let format_on_type = capabilities
            .formatter
            .format_on_type
            .ok_or_else(self.build_capability_error(&params.path))?;

        let settings = self.settings();
        let parse = self.get_parse(params.path.clone(), Some(FeatureName::Format))?;
        if !settings.as_ref().formatter().format_with_errors && parse.has_errors() {
            return Err(WorkspaceError::format_with_errors_disabled());
        }

        format_on_type(&params.path, parse, settings, params.offset)
    }

    fn fix_file(&self, params: super::FixFileParams) -> Result<FixFileResult, WorkspaceError> {
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

    fn rename(&self, params: super::RenameParams) -> Result<RenameResult, WorkspaceError> {
        let capabilities = self.get_capabilities(&params.path);
        let rename = capabilities
            .analyzer
            .rename
            .ok_or_else(self.build_capability_error(&params.path))?;

        let parse = self.get_parse(params.path.clone(), None)?;
        let result = rename(&params.path, parse, params.symbol_at, params.new_name)?;

        Ok(result)
    }

    fn rage(&self, _: RageParams) -> Result<RageResult, WorkspaceError> {
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
