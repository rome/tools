use crate::Termination;

const MAIN: &str = concat!(
    "Rome v",
    env!("CARGO_PKG_VERSION"),
    "
Available commands:
- format
- help
",
);

const FORMAT: &str = "Rome Formatter

USAGE:
    rome format [OPTIONS] <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files

OPTIONS:
    --ci                        Enable CI mode, lock files and exit with an error if the formatter would modify them
    --skip-errors               Skip over files containing syntax errors instead of returning an error
    --indent-style <tabs|space> Determine whether the formatter should use tabs or spaces for indentation (default: tabs)
    --indent-size <number>      If the indentation style is set to spaces, determine how many spaces should be used for indentation (default: 2)
";

pub(crate) fn help(command: Option<&str>) -> Result<(), Termination> {
    match command {
        None => {
            print!("{MAIN}");
            Ok(())
        }
        Some("format") => {
            print!("{FORMAT}");
            Ok(())
        }

        Some(cmd) => Err(Termination::UnknownCommandHelp {
            command: cmd.into(),
        }),
    }
}
