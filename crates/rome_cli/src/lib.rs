use pico_args::Arguments;
use rome_core::App;
use std::env;

mod commands;

const HELP: &str = concat!(
    "Rome CLI v",
    env!("CARGO_PKG_VERSION"),
    "
Available commands:
- format
- help
",
);

pub struct CliSession {
    pub app: App,
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
        Ok(Some(_cmd)) if has_help => {
            // TODO: Print command specific help
            println!("{HELP}");
        }

        Ok(Some("format")) => crate::commands::format::format(session),

        Ok(None | Some("help")) => {
            println!("{HELP}");
        }

        Ok(Some(cmd)) => {
            panic!("unknown command {cmd:?}")
        }
        Err(err) => {
            panic!("failed to parse command: {err}")
        }
    }
}
