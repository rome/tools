use std::panic::RefUnwindSafe;

use rome_analyze::AnalyzerAction;
use rome_diagnostics::Diagnostic;
use rome_formatter::{IndentStyle, Printed};
use rome_fs::RomePath;
use rome_js_syntax::{TextRange, TextSize};

use crate::{settings::WorkspaceSettings, RomeError};

pub(crate) mod server;

pub struct SupportsFeatureParams {
    pub path: RomePath,
    pub feature: FeatureName,
}

pub enum FeatureName {
    Format,
    Lint,
}

pub struct UpdateSettingsParams {
    pub settings: WorkspaceSettings,
}

pub struct OpenFileParams {
    pub path: RomePath,
    pub content: String,
    pub version: i32,
}

pub struct GetSyntaxTreeParams {
    pub path: RomePath,
}

pub struct ChangeFileParams {
    pub path: RomePath,
    pub content: String,
    pub version: i32,
}

pub struct CloseFileParams {
    pub path: RomePath,
}

pub struct PullDiagnosticsParams {
    pub path: RomePath,
}

pub struct PullActionsParams {
    pub path: RomePath,
    pub range: TextRange,
}

pub struct FormatFileParams {
    pub path: RomePath,
    pub indent_style: IndentStyle,
}

pub struct FormatRangeParams {
    pub path: RomePath,
    pub range: TextRange,
    pub indent_style: IndentStyle,
}

pub struct FormatOnTypeParams {
    pub path: RomePath,
    pub offset: TextSize,
    pub indent_style: IndentStyle,
}

pub trait Workspace: Send + Sync + RefUnwindSafe {
    /// Checks whether a certain feature is supported for a file at a given path
    fn supports_feature(&self, params: SupportsFeatureParams) -> bool;

    /// Update the global settings for this workspace
    fn update_settings(&self, params: UpdateSettingsParams) -> Result<(), RomeError>;

    /// Add a new file to the workspace
    fn open_file(&self, params: OpenFileParams) -> Result<(), RomeError>;

    // Return a textual, debug representation of the syntax tree for a given document
    fn get_syntax_tree(&self, params: GetSyntaxTreeParams) -> Result<String, RomeError>;

    /// Change the content of an open file
    fn change_file(&self, params: ChangeFileParams) -> Result<(), RomeError>;

    /// Remove a file from the workspace
    fn close_file(&self, params: CloseFileParams) -> Result<(), RomeError>;

    /// Retrieves the list of diagnostics associated with a file
    fn pull_diagnostics(&self, params: PullDiagnosticsParams)
        -> Result<Vec<Diagnostic>, RomeError>;

    /// Retrieves the list of code actions available for a given cursor
    /// position within a file
    fn pull_actions(&self, params: PullActionsParams) -> Result<Vec<AnalyzerAction>, RomeError>;

    /// Runs the given file through the formatter using the provided options
    /// and returns the resulting source code
    fn format_file(&self, params: FormatFileParams) -> Result<Printed, RomeError>;

    /// Runs a range of an open document through the formatter
    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, RomeError>;

    /// Runs a "block" ending at the specified character of an open document
    /// through the formatter
    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, RomeError>;
}

/// Convenience function for constructing a server instance of [Workspace]
pub fn server() -> Box<dyn Workspace> {
    Box::new(server::WorkspaceServer::new())
}

/// RAII guard for an open file in a workspace, takes care of closing the file
/// automatically on drop
pub struct FileGuard<'app, W: Workspace + ?Sized> {
    workspace: &'app W,
    path: RomePath,
}

impl<'app, W: Workspace + ?Sized> FileGuard<'app, W> {
    pub fn open(workspace: &'app W, params: OpenFileParams) -> Result<Self, RomeError> {
        let path = params.path.clone();
        workspace.open_file(params)?;
        Ok(Self { workspace, path })
    }

    pub fn get_syntax_tree(&self) -> Result<String, RomeError> {
        self.workspace.get_syntax_tree(GetSyntaxTreeParams {
            path: self.path.clone(),
        })
    }

    pub fn change_file(&self, version: i32, content: String) -> Result<(), RomeError> {
        self.workspace.change_file(ChangeFileParams {
            path: self.path.clone(),
            version,
            content,
        })
    }

    pub fn pull_diagnostics(&self) -> Result<Vec<Diagnostic>, RomeError> {
        self.workspace.pull_diagnostics(PullDiagnosticsParams {
            path: self.path.clone(),
        })
    }

    pub fn pull_actions(&self, range: TextRange) -> Result<Vec<AnalyzerAction>, RomeError> {
        self.workspace.pull_actions(PullActionsParams {
            path: self.path.clone(),
            range,
        })
    }

    pub fn format_file(&self, indent_style: IndentStyle) -> Result<Printed, RomeError> {
        self.workspace.format_file(FormatFileParams {
            path: self.path.clone(),
            indent_style,
        })
    }

    pub fn format_range(
        &self,
        indent_style: IndentStyle,
        range: TextRange,
    ) -> Result<Printed, RomeError> {
        self.workspace.format_range(FormatRangeParams {
            path: self.path.clone(),
            indent_style,
            range,
        })
    }

    pub fn format_on_type(
        &self,
        indent_style: IndentStyle,
        offset: TextSize,
    ) -> Result<Printed, RomeError> {
        self.workspace.format_on_type(FormatOnTypeParams {
            path: self.path.clone(),
            indent_style,
            offset,
        })
    }
}

impl<'app, W: Workspace + ?Sized> Drop for FileGuard<'app, W> {
    fn drop(&mut self) {
        self.workspace
            .close_file(CloseFileParams {
                path: self.path.clone(),
            })
            // `close_file` can only error if the file was already closed, in
            // this case it's generally better to silently ignore the error
            // than panic (especially in a drop handler)
            .ok();
    }
}
