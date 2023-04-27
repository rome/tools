#![doc = include_str!("../README.md")]
//!
//! # Module
//!
//! This is where the main CLI session starts. The module is responsible
//! to parse commands and arguments, redirect the execution of the commands and
//! execute the traversal of directory and files, based on the command that were passed.

pub use pico_args::Arguments;
use rome_console::{ColorMode, Console};
use rome_fs::OsFileSystem;
use rome_service::{App, DynRef, Workspace, WorkspaceRef};
use std::env;

mod cli_options;
mod commands;
mod configuration;
mod diagnostics;
mod execute;
mod metrics;
mod panic;
mod reports;
mod service;
mod vcs;

use crate::cli_options::ColorsArg;
use crate::commands::check::CheckCommandPayload;
use crate::commands::ci::CiCommandPayload;
use crate::commands::format::FormatCommandPayload;
pub use crate::commands::{parse_command, RomeCommand};
pub use diagnostics::CliDiagnostic;
pub(crate) use execute::{execute_mode, Execution, TraversalMode};
pub use panic::setup_panic_handler;
pub use reports::{
    formatter::{FormatterReport, FormatterReportFileDetail, FormatterReportSummary},
    Report, ReportDiagnostic, ReportDiff, ReportErrorKind, ReportKind,
};
pub use service::{open_transport, SocketTransport};

pub(crate) const VERSION: &str = match option_env!("ROME_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

/// Global context for an execution of the CLI
pub struct CliSession<'app> {
    /// Instance of [App] used by this run of the CLI
    pub app: App<'app>,
}

impl<'app> CliSession<'app> {
    pub fn new(
        workspace: &'app dyn Workspace,
        console: &'app mut dyn Console,
    ) -> Result<Self, CliDiagnostic> {
        Ok(Self {
            app: App::new(
                DynRef::Owned(Box::new(OsFileSystem)),
                console,
                WorkspaceRef::Borrowed(workspace),
            ),
        })
    }

    /// Main function to run Rome CLI
    pub fn run(self, command: RomeCommand) -> Result<(), CliDiagnostic> {
        let has_metrics = command.has_metrics();
        if has_metrics {
            crate::metrics::init_metrics();
        }

        let result = match command {
            RomeCommand::Version(_) => commands::version::full_version(self),
            RomeCommand::Rage(_) => commands::rage::rage(self),
            RomeCommand::Start => commands::daemon::start(self),
            RomeCommand::Stop => commands::daemon::stop(self),
            RomeCommand::Check {
                apply,
                apply_unsafe,
                cli_options,
                rome_configuration,
                paths,
            } => commands::check::check(
                self,
                CheckCommandPayload {
                    apply_unsafe,
                    apply,
                    cli_options,
                    configuration: rome_configuration,
                    paths,
                },
            ),
            RomeCommand::Ci {
                linter_enabled,
                formatter_enabled,
                organize_imports_enabled,
                rome_configuration,
                paths,
                cli_options,
            } => commands::ci::ci(
                self,
                CiCommandPayload {
                    linter_enabled,
                    formatter_enabled,
                    organize_imports_enabled,
                    rome_configuration,
                    paths,
                    cli_options,
                },
            ),
            RomeCommand::Format {
                javascript_formatter,
                formatter_configuration,
                stdin_file_path,
                write,
                cli_options,
                paths,
                vcs_configuration,
                files_configuration,
            } => commands::format::format(
                self,
                FormatCommandPayload {
                    javascript_formatter,
                    formatter_configuration,
                    stdin_file_path,
                    write,
                    cli_options,
                    paths,
                    vcs_configuration,
                    files_configuration,
                },
            ),
            RomeCommand::Init => commands::init::init(self),
            RomeCommand::LspProxy => commands::daemon::lsp_proxy(),
            RomeCommand::Migrate(cli_options, write) => {
                commands::migrate::migrate(self, cli_options, write)
            }
            RomeCommand::RunServer { stop_on_disconnect } => {
                commands::daemon::run_server(stop_on_disconnect)
            }
            RomeCommand::PrintSocket => commands::daemon::print_socket(),
        };

        if has_metrics {
            metrics::print_metrics();
        }

        result
    }
}

pub fn to_color_mode(color: Option<&ColorsArg>) -> ColorMode {
    match color {
        Some(ColorsArg::Off) => ColorMode::Disabled,
        Some(ColorsArg::Force) => ColorMode::Enabled,
        None => ColorMode::Auto,
    }
}
