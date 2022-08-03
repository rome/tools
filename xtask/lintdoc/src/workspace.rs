use rome_console::codespan::Severity;
use rome_formatter::Printed;
use rome_service::settings::WorkspaceSettings;
use rome_service::workspace::{
    ChangeFileParams, CloseFileParams, FixFileParams, FixFileResult, FormatFileParams,
    FormatOnTypeParams, FormatRangeParams, GetSyntaxTreeParams, OpenFileParams, PullActionsParams,
    PullActionsResult, PullDiagnosticsParams, PullDiagnosticsResult, RenameParams, RenameResult,
    SupportsFeatureParams, UpdateSettingsParams,
};
use rome_service::{RomeError, Workspace, WorkspaceExt};
use std::sync::RwLock;

/// A slim workspace, which exposes some utilities via [WorkspaceExt]
///
/// ## Warnings
///
/// This is a workspace that holds only utility functionalities and should be used only for tests
pub struct WorkspaceTest {
    pub settings: RwLock<WorkspaceSettings>,
}

impl Workspace for WorkspaceTest {
    fn supports_feature(&self, _params: SupportsFeatureParams) -> bool {
        unimplemented!()
    }

    fn update_settings(&self, params: UpdateSettingsParams) -> Result<(), RomeError> {
        let mut settings = self.settings.write().unwrap();
        *settings = params.settings;
        Ok(())
    }

    fn open_file(&self, _params: OpenFileParams) -> Result<(), RomeError> {
        unimplemented!()
    }

    fn get_syntax_tree(&self, _params: GetSyntaxTreeParams) -> Result<String, RomeError> {
        unimplemented!()
    }

    fn change_file(&self, _params: ChangeFileParams) -> Result<(), RomeError> {
        unimplemented!()
    }

    fn close_file(&self, _params: CloseFileParams) -> Result<(), RomeError> {
        unimplemented!()
    }

    fn pull_diagnostics(
        &self,
        _params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, RomeError> {
        unimplemented!()
    }

    fn pull_actions(&self, _params: PullActionsParams) -> Result<PullActionsResult, RomeError> {
        unimplemented!()
    }

    fn format_file(&self, _params: FormatFileParams) -> Result<Printed, RomeError> {
        unimplemented!()
    }

    fn format_range(&self, _params: FormatRangeParams) -> Result<Printed, RomeError> {
        unimplemented!()
    }

    fn format_on_type(&self, _params: FormatOnTypeParams) -> Result<Printed, RomeError> {
        unimplemented!()
    }

    fn fix_file(&self, _params: FixFileParams) -> Result<FixFileResult, RomeError> {
        unimplemented!()
    }

    fn rename(&self, _params: RenameParams) -> Result<RenameResult, RomeError> {
        unimplemented!()
    }
}

impl WorkspaceExt for WorkspaceTest {
    fn get_severity_from_rule_code(&self, code: &str) -> Severity {
        let settings = self.settings.read().unwrap();
        let rules = &settings.linter.rules;
        let rules = rules.as_ref().expect("rules should be populated");
        let severity = rules.get_severity_from_code(code);
        if let Some(severity) = severity {
            severity
        } else {
            Severity::Warning
        }
    }
}
