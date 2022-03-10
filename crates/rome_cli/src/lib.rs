use pico_args::Arguments;
use rome_core::App;

mod commands;

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
            app: App::new(),
            args: Arguments::from_env(),
        }
    }
}

/// Main function to run Rome CLI
pub fn run_cli(mut session: CliSession) {
    let has_help = session.args.contains("--help");
    let subcommand = session.args.subcommand();

    match subcommand.as_ref().map(Option::as_deref) {
        Ok(Some(cmd)) if has_help => crate::commands::help::help(Some(cmd)),

        Ok(Some("format")) => crate::commands::format::format(session),

        Ok(None | Some("help")) => crate::commands::help::help(None),

        Ok(Some(cmd)) => {
            panic!("unknown command {cmd:?}")
        }
        Err(err) => {
            panic!("failed to parse command: {err}")
        }
    }
}
