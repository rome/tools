use crate::global_options::{global_options, GlobalOptions};
use crate::VERSION;
use bpaf::{Bpaf, OptionParser};
use rome_service::configuration::{
    formatter_configuration, javascript::javascript_formatter, rome_configuration,
    FormatterConfiguration, JavascriptFormatter,
};
use rome_service::RomeConfiguration;
use std::path::PathBuf;

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

    /// Run various checks on a set of files
    #[bpaf(command)]
    Check {
        /// Apply safe fixes, formatting
        #[bpaf(long("apply"), switch)]
        apply: bool,
        /// Apply safe fixes and unsafe fixes, formatting and import sorting
        #[bpaf(long("apply-unsafe"), switch)]
        apply_unsafe: bool,

        #[bpaf(external, hide_usage)]
        rome_configuration: RomeConfiguration,
        #[bpaf(external, hide_usage)]
        global_options: GlobalOptions,

        /// Single file, single path or list of paths
        #[bpaf(positional::<PathBuf>("PATH"), many)]
        paths: Vec<PathBuf>,
    },
    /// Run the linter and check the formatting of a set of files
    #[bpaf(command)]
    Ci {
        /// Allow to enable or disable the formatter check.
        #[bpaf(long("formatter-enabled"), argument("true|false"), fallback(true))]
        formatter_enabled: bool,
        /// Allow to enable or disable the linter check.
        #[bpaf(long("linter-enabled"), argument("true|false"), fallback(true))]
        linter_enabled: bool,
        /// Allow to enable or disable the organize imports.
        #[bpaf(
            long("organize-imports-enabled"),
            argument("true|false"),
            fallback(true)
        )]
        organize_imports_enabled: bool,

        #[bpaf(external, hide_usage)]
        rome_configuration: RomeConfiguration,
        #[bpaf(external, hide_usage)]
        global_options: GlobalOptions,

        /// Single file, single path or list of paths
        #[bpaf(positional::<PathBuf>("PATH"), many)]
        paths: Vec<PathBuf>,
    },
    /// Run the formatter on a set of files
    #[bpaf(command)]
    Format {
        #[bpaf(external, optional, hide_usage)]
        formatter_configuration: Option<FormatterConfiguration>,

        #[bpaf(external, optional, hide_usage)]
        javascript_formatter: Option<JavascriptFormatter>,

        /// A file name with its extension to pass when reading from standard in, e.g. echo 'let a;' | rome format --stdin-file-path=file.js"
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide_usage)]
        stdin_file_path: Option<String>,

        #[bpaf(external, hide_usage)]
        global_options: GlobalOptions,

        #[bpaf(switch)]
        write: bool,

        /// Single file, single path or list of paths
        #[bpaf(positional::<PathBuf>("PATH"), many)]
        paths: Vec<PathBuf>,
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
        #[bpaf(external(global_options), hide_usage)] GlobalOptions,
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
        let result = parse_command().run_inner(Args::from(&["migrate", "--write"]));
        let help = parse_command().run_inner(Args::from(&["ci", "--help"]));

        // let result = result.unwrap_err().unwrap_stdout();

        println!("{:?}", result);
        println!("{}", help.unwrap_err().unwrap_stdout());
    }
}
