use rome_console::{markup, ConsoleExt, Markup};

use crate::{CliSession, Termination, VERSION};

const MAIN: Markup = markup! {
"Rome CLI v"{VERSION}"

"<Emphasis>"COMMANDS:"</Emphasis>"
    - "<Emphasis>"check"</Emphasis>"        Run the linter on a set of files
    - "<Emphasis>"ci"</Emphasis>"           Run the linter and check the formatting of a set of files
    - "<Emphasis>"format"</Emphasis>"       Run the formatter on a set of files
    - "<Emphasis>"init"</Emphasis>"         Bootstraps a new rome project
    - "<Emphasis>"start"</Emphasis>"        Start the Rome daemon server process
    - "<Emphasis>"stop"</Emphasis>"         Stop the Rome daemon server process
    - "<Emphasis>"lsp-proxy"</Emphasis>"    Acts as a server for the Language Server Protocol over stdin/stdout
    - "<Emphasis>"rage"</Emphasis>"         Prints information for debugging
    - "<Emphasis>"version"</Emphasis>"      Shows the Rome version information and quit
    - "<Emphasis>"help"</Emphasis>"         Prints this help message

"<Emphasis>"OPTIONS:"</Emphasis>"
    "<Dim>"--no-colors"</Dim>"              Disable the formatting of markup (print everything as plain text)
    "<Dim>"--force-colors"</Dim>"           Force the formatting of markup using ANSI, even if the console output is determined to be incompatible
    "<Dim>"--use-server"</Dim>"             Connect to a running instance of the Rome daemon server
    "<Dim>"--version"</Dim>"                Show the Rome version information and quit
    "<Dim>"--files-max-size"</Dim>"         The maximum allowed size for source code files in bytes (default: 1MB)
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
    "<Dim>"--max-diagnostics"</Dim>"             Cap the amount of diagnostics displayed (default: 20)
"
};

const FORMAT_OPTIONS: Markup = markup! {
    "
    "<Dim>"--indent-style <tabs|space>"</Dim>"              Change the indention character (default: tabs)
    "<Dim>"--indent-size <number>"</Dim>"                   If the indentation style is set to spaces, determine how many spaces should be used for indentation (default: 2)
    "<Dim>"--line-width <number>"</Dim>"                    Change how many characters the formatter is allowed to print in a single line (default: 80)
    "<Dim>"--quote-style <single|double>"</Dim>"            Changes the quotation character for strings (default: \")
    "<Dim>"--quote-properties <as-needed|preserve>"</Dim>"  Changes when properties in object should be quoted (default: as-needed)
    "<Dim>"--trailing-comma <all|es5|none>"</Dim>"          Changes trailing commas in multi-line comma-separated syntactic structures (default: all)
    "
};

const CI: Markup = markup! {
"Rome CI: Run the linter and formatter check on a set of files

"<Emphasis>"USAGE:"</Emphasis>"
    rome ci [OPTIONS] <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files

"<Emphasis>"OPTIONS:"</Emphasis>"
    "<Dim>"--formatter-enabled"</Dim>"                      Allow to enable or disable the formatter check. (default: true)
    "<Dim>"--linter-enabled"</Dim>"                         Allow to enable or disable the linter check. (default: true)
    "<Dim>"--max-diagnostics"</Dim>"                        Cap the amount of diagnostics displayed (default: 50)"
    {FORMAT_OPTIONS}
};

const FORMAT: Markup = markup! {
"Rome Formatter

"<Emphasis>"USAGE:"</Emphasis>"
    rome format [OPTIONS] <INPUTS...>

    INPUTS can be one or more filesystem path, each pointing to a single file or an entire directory to be searched recursively for supported files

"<Emphasis>"OPTIONS:"</Emphasis>"
    "<Dim>"--write"</Dim>"                                  Edit the files in place (beware!) instead of printing the diff to the console
    "<Dim>"--skip-errors"</Dim>"                            Skip over files containing syntax errors instead of emitting an error diagnostic.
    "<Dim>"--max-diagnostics"</Dim>"                        Cap the amount of diagnostics displayed (default: 50)"
    {FORMAT_OPTIONS}
   ""<Dim>"--stdin-file-path <string>"</Dim>"               A file name with its extension to pass when reading from standard in, e.g. echo 'let a;' | rome format --stdin-file-path file.js
"
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

const START_LSP_PROXY: Markup = markup! {
"Rome lsp-proxy: Acts as a server for the Language Server Protocol over stdin/stdout

	"<Emphasis>"USAGE:"</Emphasis>"
		rome lsp-proxy"
};

const RAGE: Markup = markup! {
"Rome rage: Prints information for debugging

"<Emphasis>"USAGE:"</Emphasis>"
    rome rage"
};

const VERSION_HELP_TEXT: Markup = markup! {
"Rome version: Show the Rome version information

"<Emphasis>"USAGE:"</Emphasis>"
    rome version"
};

pub(crate) fn help(mut session: CliSession, command: Option<&str>) -> Result<(), Termination> {
    let help_text = match command {
        Some("help") | None => MAIN,
        Some("check") => CHECK,
        Some("ci") => CI,
        Some("format") => FORMAT,
        Some("init") => INIT,
        Some("start") => START,
        Some("stop") => STOP,
        Some("lsp-proxy") => START_LSP_PROXY,
        Some("version") => VERSION_HELP_TEXT,
        Some("rage") => RAGE,

        Some(cmd) => {
            return Err(Termination::UnknownCommandHelp {
                command: cmd.into(),
            })
        }
    };

    session.app.console.log(help_text);
    Ok(())
}
