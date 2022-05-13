use crate::Termination;

const MAIN_HEAD: &str = "Rome v";
const MAIN_BODY: &str = "
Available commands:
- check
- ci
- format
- help
";

const CHECK: &str = "Rome Check: Run the linter on a set of files

USAGE:
    rome check <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files
";

const CI: &str = "Rome CI: Run the linter and formatter check on a set of files

USAGE:
    rome ci [OPTIONS] <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files

OPTIONS:
    --indent-style <tabs|space>   Determine whether the formatter should use tabs or spaces for indentation (default: tabs)
    --indent-size <number>        If the indentation style is set to spaces, determine how many spaces should be used for indentation (default: 2)
    --line-width <number>         Determine how many characters the formatter is allowed to print in a single line (default: 80)
    --quote-style <single|double> Determine whether the formatter should use single or double quotes for strings (default: double)
";

const FORMAT: &str = "Rome Formatter

USAGE:
    rome format [OPTIONS] <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files

OPTIONS:
    --write                       Write the output of the formatter to the files instead of printing the diff to the console
    --skip-errors                 Skip over files containing syntax errors instead of returning an error
    --indent-style <tabs|space>   Determine whether the formatter should use tabs or spaces for indentation (default: tabs)
    --indent-size <number>        If the indentation style is set to spaces, determine how many spaces should be used for indentation (default: 2)
    --line-width <number>         Determine how many characters the formatter is allowed to print in a single line (default: 80)
    --quote-style <single|double> Determine whether the formatter should use single or double quotes for strings (default: double)
";

pub(crate) fn help(command: Option<&str>) -> Result<(), Termination> {
    match command {
        Some("help") | None => {
            print!(
                "{MAIN_HEAD}{}{MAIN_BODY}",
                option_env!("ROME_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))
            );
            Ok(())
        }
        Some("check") => {
            print!("{CHECK}");
            Ok(())
        }
        Some("ci") => {
            print!("{CI}");
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
