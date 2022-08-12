//! This is where the main CLI session starts. The module is responsible
//! to parse commands and arguments, redirect the execution of the commands and
//! execute the traversal of directory and files, based on the command that were passed.

pub use pico_args::Arguments;
use rome_console::EnvConsole;
use rome_flags::FeatureFlags;
use rome_fs::OsFileSystem;
use rome_service::{App, DynRef, Workspace, WorkspaceRef};

mod commands;
mod execute;
mod metrics;
mod panic;
mod termination;
mod traversal;

pub(crate) use execute::{execute_mode, ExecutionMode};
pub use panic::setup_panic_handler;
pub use termination::Termination;

/// Global context for an execution of the CLI
pub struct CliSession<'app> {
    /// Instance of [App] used by this run of the CLI
    pub app: App<'app>,
    /// List of command line arguments
    pub args: Arguments,
}

impl<'app> CliSession<'app> {
    pub fn new(workspace: &'app dyn Workspace, mut args: Arguments) -> Self {
        let no_colors = args.contains("--no-colors");
        Self {
            app: App::new(
                DynRef::Owned(Box::new(OsFileSystem)),
                DynRef::Owned(Box::new(EnvConsole::new(no_colors))),
                WorkspaceRef::Borrowed(workspace),
            ),
            args,
        }
    }

    /// Main function to run Rome CLI
    pub fn run(mut self) -> Result<(), Termination> {
        let has_metrics = self.args.contains("--show-metrics");
        if has_metrics {
            crate::metrics::init_metrics();
        }

        if self.args.contains("--unstable") {
            rome_flags::set_unstable_flags(FeatureFlags::ALL);
        }

        let has_help = self.args.contains("--help");
        let subcommand = self
            .args
            .subcommand()
            .map_err(|source| Termination::ParseError {
                argument: "<command>",
                source,
            })?;

        // True if the command line did not contain any arguments beside the subcommand
        let is_empty = self.args.clone().finish().is_empty();

        let result = match subcommand.as_deref() {
            // Print the help for the subcommand if it was called with `--help`
            Some(cmd) if has_help => crate::commands::help::help(self, Some(cmd)),

            Some("check") if !is_empty => crate::commands::check::check(self),
            Some("ci") if !is_empty => crate::commands::ci::ci(self),
            Some("format") if !is_empty => crate::commands::format::format(self),

            // Print the help for known commands called without any arguments, and exit with an error
            Some(cmd @ ("check" | "ci" | "format")) => {
                crate::commands::help::help(self, Some(cmd))?;
                Err(Termination::EmptyArguments)
            }

            Some("init") => crate::commands::init::init(self),

            // Print the general help if no subcommand was specified / the subcommand is `help`
            None | Some("help") => crate::commands::help::help(self, None),

            Some(cmd) => Err(Termination::UnknownCommand {
                command: cmd.into(),
            }),
        };

        if has_metrics {
            crate::metrics::print_metrics();
        }

        result
    }
}
