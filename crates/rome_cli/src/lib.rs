use pico_args::Arguments;
use rome_core::App;

mod commands;
mod metrics;
mod panic;
mod termination;

pub use panic::setup_panic_handler;
pub use termination::Termination;

/// Global context for an execution of the CLI
pub struct CliSession<'app> {
    /// Instance of [App] used by this run of the CLI
    pub app: App<'app>,
    /// List of command line arguments
    pub args: Arguments,
}

impl CliSession<'static> {
    pub fn from_env() -> Self {
        Self {
            app: App::from_env(),
            args: Arguments::from_env(),
        }
    }
}

/// Main function to run Rome CLI
pub fn run_cli(mut session: CliSession) -> Result<(), Termination> {
    let has_metrics = session.args.contains("--show-metrics");
    if has_metrics {
        crate::metrics::init_metrics();
    }

    let has_help = session.args.contains("--help");
    let subcommand = session
        .args
        .subcommand()
        .map_err(|source| Termination::ParseError {
            argument: "<command>",
            source,
        })?;

    match subcommand.as_deref() {
        Some(cmd) if has_help => crate::commands::help::help(Some(cmd)),

        Some("format") => {
            let result = crate::commands::format::format(session);

            if has_metrics {
                crate::metrics::print_metrics();
            }

            result
        }

        None | Some("help") => crate::commands::help::help(None),

        Some(cmd) => Err(Termination::UnknownCommand {
            command: cmd.into(),
        }),
    }
}
