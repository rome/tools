use std::fmt::{self, Debug, Display, Formatter};

use pico_args::Arguments;
use rome_core::App;

mod commands;
mod metrics;
mod panic;

pub use panic::setup_panic_handler;

/// Global context for an execution of the CLI
pub struct CliSession {
    /// Instance of [App] used by this run of the CLI
    pub app: App,
    /// List of command line arguments
    pub args: Arguments,
}

impl CliSession {
    pub fn from_env() -> Self {
        Self {
            app: App::from_env(),
            args: Arguments::from_env(),
        }
    }
}

/// Error message returned by the CLI when it aborts with an error
#[derive(PartialEq, Eq)]
pub struct Termination(pub(crate) String);

impl<T: Display> From<T> for Termination {
    fn from(msg: T) -> Self {
        Self(msg.to_string())
    }
}

impl Debug for Termination {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

/// Main function to run Rome CLI
pub fn run_cli(mut session: CliSession) -> Result<(), Termination> {
    let has_metrics = session.args.contains("--show-metrics");
    if has_metrics {
        crate::metrics::init_metrics();
    }

    let has_help = session.args.contains("--help");
    let subcommand = session.args.subcommand();

    match subcommand.as_ref().map(Option::as_deref) {
        Ok(Some(cmd)) if has_help => {
            crate::commands::help::help(Some(cmd));
            Ok(())
        }

        Ok(Some("format")) => {
            let result = crate::commands::format::format(session);

            if has_metrics {
                crate::metrics::print_metrics();
            }

            result
        }

        Ok(None | Some("help")) => {
            crate::commands::help::help(None);
            Ok(())
        }

        Ok(Some(cmd)) => Err(Termination(format!("unknown command {cmd:?}"))),
        Err(err) => Err(Termination(format!("failed to parse command: {err}"))),
    }
}
