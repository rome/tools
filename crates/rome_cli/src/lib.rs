#![doc = include_str!("../README.md")]
//!
//! # Module
//!
//! This is where the main CLI session starts. The module is responsible
//! to parse commands and arguments, redirect the execution of the commands and
//! execute the traversal of directory and files, based on the command that were passed.

use std::str::FromStr;

pub use pico_args::Arguments;
use rome_console::{ColorMode, Console};
use rome_fs::OsFileSystem;
use rome_service::{App, DynRef, Workspace, WorkspaceRef};

mod commands;
mod configuration;
mod diagnostics;
mod execute;
mod metrics;
mod panic;
mod parse_arguments;
mod reports;
mod service;
mod traversal;

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
    /// List of command line arguments
    pub args: Arguments,
}

impl<'app> CliSession<'app> {
    pub fn new(
        workspace: &'app dyn Workspace,
        args: Arguments,
        console: &'app mut dyn Console,
    ) -> Result<Self, CliDiagnostic> {
        Ok(Self {
            app: App::new(
                DynRef::Owned(Box::new(OsFileSystem)),
                console,
                WorkspaceRef::Borrowed(workspace),
            ),
            args,
        })
    }

    /// Main function to run Rome CLI
    pub fn run(mut self) -> Result<(), CliDiagnostic> {
        let has_metrics = self.args.contains("--show-metrics");
        if has_metrics {
            crate::metrics::init_metrics();
        }

        let has_help = self.args.contains("--help");
        let subcommand = self
            .args
            .subcommand()
            .map_err(|source| CliDiagnostic::parse_error("<command>", source))?;

        // True if the command line did not contain any arguments beside the subcommand
        let is_empty = self.args.clone().finish().is_empty();

        let result = match subcommand.as_deref() {
            // Print the help for the subcommand if it was called with `--help`
            Some(cmd) if has_help => commands::help::help(self, Some(cmd)),

            Some("check") if !is_empty => commands::check::check(self),
            Some("ci") if !is_empty => commands::ci::ci(self),
            Some("format") if !is_empty => commands::format::format(self),

            Some("start") => commands::daemon::start(self),
            Some("stop") => commands::daemon::stop(self),
            Some("lsp-proxy") => commands::daemon::lsp_proxy(),

            // Internal commands
            Some("__run_server") => commands::daemon::run_server(self),
            Some("__print_socket") => commands::daemon::print_socket(),

            // Print the help for known commands called without any arguments, and exit with an error
            Some(cmd @ ("check" | "ci" | "format")) => {
                commands::help::help(self, Some(cmd))?;
                Err(CliDiagnostic::empty_arguments())
            }

            Some("init") => commands::init::init(self),

            Some("version") => commands::version::full_version(self),
            Some("rage") => commands::rage::rage(self),
            None if self.args.contains("--version") => commands::version::brief_version(self),

            // Print the general help if no subcommand was specified / the subcommand is `help`
            None | Some("help") => commands::help::help(self, None),

            Some(cmd) => Err(CliDiagnostic::unknown_command(cmd)),
        };

        if has_metrics {
            metrics::print_metrics();
        }

        result
    }
}

enum ColorsArg {
    Off,
    Force,
}

impl FromStr for ColorsArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(Self::Off),
            "force" => Ok(Self::Force),
            _ => Err(format!(
                "value {s:?} is not valid for the --colors argument"
            )),
        }
    }
}

pub fn color_from_arguments(args: &mut Arguments) -> Result<ColorMode, CliDiagnostic> {
    let colors = args
        .opt_value_from_str("--colors")
        .map_err(|source| CliDiagnostic::parse_error("--colors", source))?;
    Ok(match colors {
        Some(ColorsArg::Off) => ColorMode::Disabled,
        Some(ColorsArg::Force) => ColorMode::Enabled,
        None => ColorMode::Auto,
    })
}
