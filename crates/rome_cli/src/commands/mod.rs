use crate::global_options::{global_options, GlobalOptions};
use crate::VERSION;
use bpaf::{Bpaf, OptionParser};
use rome_service::configuration::{formatter_configuration, FormatterConfiguration};

pub(crate) mod check;
pub(crate) mod ci;
pub(crate) mod daemon;
pub(crate) mod format;
pub(crate) mod help;
pub(crate) mod init;
pub(crate) mod migrate;
pub(crate) mod rage;
pub(crate) mod version;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version(VERSION))]
pub(crate) enum Command {
    /// Shows the Rome version information and quit
    #[bpaf(command)]
    Version,

    #[bpaf(command)]
    /// Prints information for debugging
    Rage,
    /// Start the Rome daemon server process
    #[bpaf(command)]
    Start,

    /// Stop the Rome daemon server process
    #[bpaf(command)]
    Stop,

    /// Run the linter on a set of files
    #[bpaf(command)]
    Check,
    /// Run the linter and check the formatting of a set of files
    #[bpaf(command)]
    Ci,
    /// Run the formatter on a set of files
    #[bpaf(command)]
    Format {
        #[bpaf(external(global_options))]
        global_options: GlobalOptions,

        /// A file name with its extension to pass when reading from standard in, e.g. echo 'let a;' | rome format --stdin-file-path=file.js"
        #[bpaf(long("stdin-file-path"), argument("PATH"))]
        stdin_file_path: Option<String>,

        #[bpaf(external())]
        formatter_configuration: FormatterConfiguration,
    },
    /// Bootstraps a new rome project
    #[bpaf(command)]
    Init,
    /// Prints this help message
    #[bpaf(command)]
    Help,
    /// Acts as a server for the Language Server Protocol over stdin/stdout
    #[bpaf(command)]
    LspProxy,
    /// It updates the configuration when there are breaking changes
    #[bpaf(command)]
    Migrate(
        #[bpaf(external(global_options))] GlobalOptions,
        /// Writes the new configuration file to disk
        #[bpaf(long("write"), switch)]
        bool,
    ),
}

pub(crate) fn parse_command() -> OptionParser<Command> {
    command().header("Rome CLI").usage("rome COMMAND [ARG]")
}

#[cfg(test)]
mod test {
    use crate::commands::parse_command;
    use bpaf::Args;

    #[test]
    fn version() {
        let result =
            parse_command().run_inner(Args::from(&["migrate", "--colors=force", "--write"]));
        let help = parse_command().run_inner(Args::from(&["format", "--help"]));

        // let result = result.unwrap_err().unwrap_stdout();

        println!("{:?}", result);
        println!("{}", help.unwrap_err().unwrap_stdout());
    }
}
