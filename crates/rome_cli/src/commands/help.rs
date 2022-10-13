use rome_console::{markup, ConsoleExt, Markup};

use crate::{CliSession, Termination};

const VERSION: &str = match option_env!("ROME_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

const MAIN: Markup = markup! {
"Rome CLI v"{VERSION}"

"<Emphasis>"COMMANDS:"</Emphasis>"
    - "<Emphasis>"check"</Emphasis>"        Run the linter on a set of files
    - "<Emphasis>"ci"</Emphasis>"           Run the linter and formatter check on a set of files
    - "<Emphasis>"format"</Emphasis>"       Run the formatter on a set of files
    - "<Emphasis>"help"</Emphasis>"         Prints this help message
    - "<Emphasis>"init"</Emphasis>"         Bootstraps a new rome project
    - "<Emphasis>"start"</Emphasis>"        Start the Rome daemon server process
    - "<Emphasis>"stop"</Emphasis>"         Stop the Rome daemon server process

"<Emphasis>"OPTIONS:"</Emphasis>"
    "<Dim>"--no-colors"</Dim>"      Disable the formatting of markup (print everything as plain text)
    "<Dim>"--use-server"</Dim>"     Connect to a running instance of the Rome daemon server
"
};

const CHECK: Markup = markup! {
    <Emphasis>"Rome Check"</Emphasis>": Run the linter on a set of files

"<Emphasis>"USAGE:"</Emphasis>"
    rome check <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files

"<Emphasis>"OPTIONS:"</Emphasis>"
    "<Dim>"--apply"</Dim>"                       Apply safe fixes
    "<Dim>"--apply-suggested"</Dim>"             Apply safe and suggested fixes
    "<Dim>"--max-diagnostics"</Dim>"             Cap the amount of diagnostics displayed - default 20
"
};

const FORMAT_OPTIONS: Markup = markup! {
    "
    "<Dim>"--write"</Dim>"                                  Edit the files in place (beware!) instead of printing the diff to the console
    "<Dim>"--skip-errors"</Dim>"                            Skip over files containing syntax errors instead of emitting an error diagnostic.
    "<Dim>"--indent-style <tabs|space>"</Dim>"              Change the indention character (default: tabs)
    "<Dim>"--indent-size <number>"</Dim>"                   If the indentation style is set to spaces, determine how many spaces should be used for indentation (default: 2)
    "<Dim>"--line-width <number>"</Dim>"                    Change how many characters the formatter is allowed to print in a single line (default: 80)
    "<Dim>"--quote-style <single|double>"</Dim>"            Changes the quotation character for strings (default: \")
    "<Dim>"--quote-properties <as-needed|preserve>"</Dim>"  Changes when properties in object should be quoted (default: as-needed)
    "<Dim>"--stdin-file-path <string>"</Dim>"                A file name with its extension to pass when reading from standard in, e.g. echo 'let a;' | rome format --stdin-file-path file.js
    "
};

const CI: Markup = markup! {
"Rome CI: Run the linter and formatter check on a set of files

"<Emphasis>"USAGE:"</Emphasis>"
    rome ci [OPTIONS] <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files

"<Emphasis>"OPTIONS:"</Emphasis>
    {FORMAT_OPTIONS}
};

const FORMAT: Markup = markup! {
"Rome Formatter

"<Emphasis>"USAGE:"</Emphasis>"
    rome format [OPTIONS] <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files

"<Emphasis>"OPTIONS:"</Emphasis>""
    {FORMAT_OPTIONS}
};

const INIT: Markup = markup! {
"Rome init: bootstraps a new rome project"

};

const START: Markup = markup! {
"Rome start: Start the Rome daemon server process

"<Emphasis>"USAGE:"</Emphasis>"
    rome start"
};

const STOP: Markup = markup! {
"Rome stop: Stop the Rome daemon server process

"<Emphasis>"USAGE:"</Emphasis>"
    rome stop"
};

pub(crate) fn help(mut session: CliSession, command: Option<&str>) -> Result<(), Termination> {
    match command {
        Some("help") | None => {
            session.app.console.log(MAIN);
            Ok(())
        }
        Some("check") => {
            session.app.console.log(CHECK);
            Ok(())
        }
        Some("ci") => {
            session.app.console.log(CI);
            Ok(())
        }
        Some("format") => {
            session.app.console.log(FORMAT);
            Ok(())
        }
        Some("init") => {
            session.app.console.log(INIT);
            Ok(())
        }
        Some("start") => {
            session.app.console.log(START);
            Ok(())
        }
        Some("stop") => {
            session.app.console.log(STOP);
            Ok(())
        }

        Some(cmd) => Err(Termination::UnknownCommandHelp {
            command: cmd.into(),
        }),
    }
}
