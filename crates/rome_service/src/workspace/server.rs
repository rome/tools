use std::sync::Mutex;

use rome_formatter::Printed;
use salsa::{Durability, ParallelDatabase, Snapshot};

use crate::{
    database::{Analyzer, Document, Formatter, Inputs, Parser, WorkspaceDatabase},
    RomeError, Workspace,
};

use super::{
    ChangeFileParams, CloseFileParams, FeatureName, FixFileResult, FormatFileParams,
    FormatOnTypeParams, FormatRangeParams, GetSyntaxTreeParams, OpenFileParams, PullActionsParams,
    PullActionsResult, PullDiagnosticsParams, PullDiagnosticsResult, RenameResult,
    SupportsFeatureParams, UpdateSettingsParams,
};

#[derive(Default)]
pub(super) struct WorkspaceServer {
    /// Salsa database holding most of the state for the workspace
    database: Mutex<WorkspaceDatabase>,
}

impl WorkspaceServer {
    /// Returns a thread-local handle to the database
    fn database(&self) -> Snapshot<WorkspaceDatabase> {
        self.database.lock().unwrap().snapshot()
    }
}

impl Workspace for WorkspaceServer {
    fn supports_feature(&self, params: SupportsFeatureParams) -> bool {
        let database = self.database();
        let features = database.language_features(());
        let settings = database.settings(());

        let capabilities = features.get_capabilities(&params.path);
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
        let mut database = self.database.lock().unwrap();
        database.set_settings_with_durability((), params.settings, Durability::MEDIUM);
        Ok(())
    }

    /// Add a new file to the workspace
    fn open_file(&self, params: OpenFileParams) -> Result<(), RomeError> {
        let mut database = self.database.lock().unwrap();

        database.set_document(
            params.path,
            Document {
                content: params.content,
                version: params.version,
            },
        );

        Ok(())
    }

    /// Change the content of an open file
    fn change_file(&self, params: ChangeFileParams) -> Result<(), RomeError> {
        let mut database = self.database.lock().unwrap();

        let document = database.remove_document(params.path.clone());
        debug_assert!(params.version > document.version);

        database.set_document(
            params.path,
            Document {
                content: params.content,
                version: params.version,
            },
        );

        Ok(())
    }

    /// Remove a file from the workspace
    fn close_file(&self, params: CloseFileParams) -> Result<(), RomeError> {
        let mut database = self.database.lock().unwrap();
        database.remove_document(params.path);
        Ok(())
    }

    fn get_syntax_tree(&self, params: GetSyntaxTreeParams) -> Result<String, RomeError> {
        self.database().debug_print(params.path)
    }

    /// Retrieves the list of diagnostics associated with a file
    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, RomeError> {
        self.database().lint(params.path, params.categories)
    }

    /// Retrieves the list of code actions available for a given cursor
    /// position within a file
    fn pull_actions(&self, params: PullActionsParams) -> Result<PullActionsResult, RomeError> {
        self.database().code_actions(params.path, params.range)
    }

    /// Runs the given file through the formatter using the provided options
    /// and returns the resulting source code
    fn format_file(&self, params: FormatFileParams) -> Result<Printed, RomeError> {
        self.database().format(params.path, params.indent_style)
    }

    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, RomeError> {
        self.database()
            .format_range(params.path, params.indent_style, params.range)
    }

    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, RomeError> {
        self.database()
            .format_on_type(params.path, params.indent_style, params.offset)
    }

    fn fix_file(&self, params: super::FixFileParams) -> Result<FixFileResult, RomeError> {
        self.database().fix_all(params.path)
    }

    fn rename(&self, params: super::RenameParams) -> Result<RenameResult, RomeError> {
        self.database()
            .rename(params.path, params.symbol_at, params.new_name)
    }
}
