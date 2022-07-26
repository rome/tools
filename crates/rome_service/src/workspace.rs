//! The [Workspace] is the main entry point for high-level clients (the Rome
//! CLI and Language Server) into the various language-specific services of the
//! Rome toolchain (parser, formatter, analyzer ...)
//!
//! # Documents
//!
//! A [Workspace] instance operates on a set of open documents managed by one
//! or multiple clients, and provides language services for these documents
//! like diagnostics, code actions or formatting in a language independent way.
//!
//! In this regard the [Workspace] trait shares many similarities with the
//! Language Server Protocol, and in the context of the Language Server the
//! state of the [Workspace] instance is intended to closely mirror the state
//! of the actual in-editor workspace (the set of documents open in the
//! [Workspace] is the set of files currently open in the editor)
//!
//! In the context of the CLI most commands will generally work on batches of
//! files, and as such the set of "open documents" instead corresponds to the
//! list of files the CLI is currently actively processing
//!
//! # State
//!
//! A [Workspace] instance is stateful: this is not reflected on the trait (all
//! methods take an immutable `&self` borrow) because the interface is also
//! required to be thread-safe ([Send] + [Sync]), but the workspace is allowed
//! to internally cache data across calls (this is in fact the main reason for
//! the use of the "open documents" set, those documents can serve as
//! conceptual garbage collection roots to manage the caching and eviction of
//! parse trees, intermediate analysis data or diagnostics)
//!
//! # Implementations
//!
//! Currently the [Workspace] trait is implemented for a single `WorkspaceServer`
//! type. However it is eventually intended to also be implemented for a
//! potential `WorkspaceClient` type and to operate on a remote workspace
//! server through a transport layer. This would allow the CLI and Language
//! Server process to share a the same [Workspace] instance in a common daemon
//! process for instance
//!
//! # Errors
//!
//! Because of the aforementioned client-server abstraction, the [Workspace]
//! is designed to let any operation fail: all methods return a [Result] with a
//! [RomeError] enum wrapping the underlying issue. Some common errors are:
//!
//! - [RomeError::NotFound]: This error is returned when an operation is being
//! run on a path that doesn't correspond to any open document: either the
//! document has been closed or the client didn't open it in the first place
//! - [RomeError::SourceFileNotSupported]: This error is returned when an
//! operation could not be completed because the language associated with the
//! document does not implement the required capability: for instance trying to
//! format a file with a language that does not have a formatter

use crate::settings::WorkspaceSettings;
use crate::RomeError;
use rome_analyze::ActionCategory;
pub use rome_analyze::RuleCategories;
use rome_diagnostics::{CodeSuggestion, Diagnostic};
use rome_formatter::{IndentStyle, Printed};
use rome_fs::RomePath;
use rome_js_syntax::{TextRange, TextSize};
use rome_text_edit::Indel;
use std::{borrow::Cow, panic::RefUnwindSafe};

pub(crate) mod server;

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct SupportsFeatureParams {
    pub path: RomePath,
    pub feature: FeatureName,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum FeatureName {
    Format,
    Lint,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct UpdateSettingsParams {
    pub settings: WorkspaceSettings,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct OpenFileParams {
    pub path: RomePath,
    pub content: String,
    pub version: i32,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct GetSyntaxTreeParams {
    pub path: RomePath,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ChangeFileParams {
    pub path: RomePath,
    pub content: String,
    pub version: i32,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct CloseFileParams {
    pub path: RomePath,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct PullDiagnosticsParams {
    pub path: RomePath,
    pub categories: RuleCategories,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct PullDiagnosticsResult {
    pub diagnostics: Vec<Diagnostic>,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct PullActionsParams {
    pub path: RomePath,
    pub range: TextRange,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct PullActionsResult {
    pub actions: Vec<CodeAction>,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct CodeAction {
    pub category: ActionCategory,
    pub rule_name: Cow<'static, str>,
    pub suggestion: CodeSuggestion,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct FormatFileParams {
    pub path: RomePath,
    pub indent_style: IndentStyle,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct FormatRangeParams {
    pub path: RomePath,
    pub range: TextRange,
    pub indent_style: IndentStyle,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct FormatOnTypeParams {
    pub path: RomePath,
    pub offset: TextSize,
    pub indent_style: IndentStyle,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct FixFileParams {
    pub path: RomePath,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct FixFileResult {
    /// New source code for the file with all fixes applied
    pub code: String,
    /// List of all the code actions applied to the file
    pub actions: Vec<FixAction>,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct FixAction {
    /// Name of the rule that emitted this code action
    pub rule_name: Cow<'static, str>,
    /// Source range at which this action was applied
    pub range: TextRange,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct RenameParams {
    pub path: RomePath,
    pub symbol_at: TextSize,
    pub new_name: String,
}

#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct RenameResult {
    /// Range of source code modified by this rename operation
    pub range: TextRange,
    /// List of text edit operations to apply on the source code
    pub indels: Vec<Indel>,
}

pub trait Workspace: Send + Sync + RefUnwindSafe {
    /// Checks whether a certain feature is supported. There are different conditions:
    /// - Rome doesn't recognize a file, so it can provide the feature;
    /// - the feature is disabled inside the configuration;
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

    /// Retrieves the list of diagnostics associated to a file
    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, RomeError>;

    /// Retrieves the list of code actions available for a given cursor
    /// position within a file
    fn pull_actions(&self, params: PullActionsParams) -> Result<PullActionsResult, RomeError>;

    /// Runs the given file through the formatter using the provided options
    /// and returns the resulting source code
    fn format_file(&self, params: FormatFileParams) -> Result<Printed, RomeError>;

    /// Runs a range of an open document through the formatter
    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, RomeError>;

    /// Runs a "block" ending at the specified character of an open document
    /// through the formatter
    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, RomeError>;

    /// Return the content of the file with all safe code actions applied
    fn fix_file(&self, params: FixFileParams) -> Result<FixFileResult, RomeError>;

    /// Return the content of the file after renaming a symbol
    fn rename(&self, params: RenameParams) -> Result<RenameResult, RomeError>;
}

/// Convenience function for constructing a server instance of [Workspace]
pub fn server() -> Box<dyn Workspace> {
    Box::new(server::WorkspaceServer::new())
}

/// [RAII](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization)
/// guard for an open file in a workspace, takes care of closing the file
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

    pub fn pull_diagnostics(
        &self,
        categories: RuleCategories,
    ) -> Result<PullDiagnosticsResult, RomeError> {
        self.workspace.pull_diagnostics(PullDiagnosticsParams {
            path: self.path.clone(),
            categories,
        })
    }

    pub fn pull_actions(&self, range: TextRange) -> Result<PullActionsResult, RomeError> {
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

    pub fn fix_file(&self) -> Result<FixFileResult, RomeError> {
        self.workspace.fix_file(FixFileParams {
            path: self.path.clone(),
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
