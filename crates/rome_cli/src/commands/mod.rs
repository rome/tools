use crate::cli_options::{cli_options, CliOptions, ColorsArg};
use crate::VERSION;
use bpaf::{Bpaf, OptionParser};
use rome_service::configuration::vcs::VcsConfiguration;
use rome_service::configuration::{
    files_configuration, formatter_configuration, javascript::javascript_formatter,
    rome_configuration, vcs::vcs_configuration, FilesConfiguration, FormatterConfiguration,
    JavascriptFormatter,
};
use rome_service::RomeConfiguration;
use std::ffi::OsString;

pub(crate) mod check;
pub(crate) mod ci;
pub(crate) mod daemon;
pub(crate) mod format;
pub(crate) mod init;
pub(crate) mod migrate;
pub(crate) mod rage;
pub(crate) mod version;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version(VERSION))]
pub enum RomeCommand {
    /// Shows the Rome version information and quit
    #[bpaf(command)]
    Version(#[bpaf(external(cli_options), hide_usage)] CliOptions),

    #[bpaf(command)]
    /// Prints information for debugging
    Rage(#[bpaf(external(cli_options), hide_usage)] CliOptions),
    /// Start the Rome daemon server process
    #[bpaf(command)]
    Start,

    /// Stop the Rome daemon server process
    #[bpaf(command)]
    Stop,

    /// Run various checks on a set of files.
    #[bpaf(command)]
    Check {
        /// Apply safe fixes, formatting
        #[bpaf(long("apply"), switch)]
        apply: bool,
        /// Apply safe fixes and unsafe fixes, formatting and import sorting
        #[bpaf(long("apply-unsafe"), switch)]
        apply_unsafe: bool,
        #[bpaf(external, hide_usage, optional)]
        rome_configuration: Option<RomeConfiguration>,
        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,
        /// Single file, single path or list of paths
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },
    /// Run the formatter on a set of files.
    #[bpaf(command)]
    Format {
        #[bpaf(external, optional, hide_usage)]
        formatter_configuration: Option<FormatterConfiguration>,

        #[bpaf(external, optional, hide_usage)]
        javascript_formatter: Option<JavascriptFormatter>,

        #[bpaf(external, optional, hide_usage)]
        vcs_configuration: Option<VcsConfiguration>,

        #[bpaf(external, optional, hide_usage)]
        files_configuration: Option<FilesConfiguration>,

        /// A file name with its extension to pass when reading from standard in, e.g. echo 'let a;' | rome format --stdin-file-path=file.js"
        #[bpaf(long("stdin-file-path"), argument("PATH"), hide_usage)]
        stdin_file_path: Option<String>,

        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        #[bpaf(switch)]
        write: bool,

        /// Single file, single path or list of paths
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },
    /// Command to use in CI environments. Run various checks of a set of files.
    #[bpaf(command)]
    Ci {
        /// Allow to enable or disable the formatter check.
        #[bpaf(long("formatter-enabled"), argument("true|false"), optional)]
        formatter_enabled: Option<bool>,
        /// Allow to enable or disable the linter check.
        #[bpaf(long("linter-enabled"), argument("true|false"), optional)]
        linter_enabled: Option<bool>,
        /// Allow to enable or disable the organize imports.
        #[bpaf(long("organize-imports-enabled"), argument("true|false"), optional)]
        organize_imports_enabled: Option<bool>,

        #[bpaf(external, hide_usage)]
        rome_configuration: RomeConfiguration,
        #[bpaf(external, hide_usage)]
        cli_options: CliOptions,

        /// Single file, single path or list of paths
        #[bpaf(positional("PATH"), many)]
        paths: Vec<OsString>,
    },

    /// Bootstraps a new rome project. Creates a configuration file with some defaults.
    #[bpaf(command)]
    Init,
    /// Acts as a server for the Language Server Protocol over stdin/stdout
    #[bpaf(command)]
    LspProxy,
    /// It updates the configuration when there are breaking changes
    #[bpaf(command)]
    Migrate(
        #[bpaf(external(cli_options), hide_usage)] CliOptions,
        /// Writes the new configuration file to disk
        #[bpaf(long("write"), switch)]
        bool,
    ),

    #[bpaf(command, hide)]
    RunServer {
        #[bpaf(long("stop-on-disconnect"), hide_usage)]
        stop_on_disconnect: bool,
    },
    #[bpaf(command, hide)]
    PrintSocket,
}

impl RomeCommand {
    pub const fn get_color(&self) -> Option<&ColorsArg> {
        match self {
            RomeCommand::Version(cli_options) => cli_options.colors.as_ref(),
            RomeCommand::Rage(cli_options) => cli_options.colors.as_ref(),
            RomeCommand::Start => None,
            RomeCommand::Stop => None,
            RomeCommand::Check { cli_options, .. } => cli_options.colors.as_ref(),
            RomeCommand::Ci { cli_options, .. } => cli_options.colors.as_ref(),
            RomeCommand::Format { cli_options, .. } => cli_options.colors.as_ref(),
            RomeCommand::Init => None,
            RomeCommand::LspProxy => None,
            RomeCommand::Migrate(cli_options, _) => cli_options.colors.as_ref(),
            RomeCommand::RunServer { .. } => None,
            RomeCommand::PrintSocket => None,
        }
    }

    pub const fn should_use_server(&self) -> bool {
        match self {
            RomeCommand::Version(cli_options) => cli_options.use_server,
            RomeCommand::Rage(cli_options) => cli_options.use_server,
            RomeCommand::Start => false,
            RomeCommand::Stop => false,
            RomeCommand::Check { cli_options, .. } => cli_options.use_server,
            RomeCommand::Ci { cli_options, .. } => cli_options.use_server,
            RomeCommand::Format { cli_options, .. } => cli_options.use_server,
            RomeCommand::Init => false,
            RomeCommand::LspProxy => false,
            RomeCommand::Migrate(cli_options, _) => cli_options.use_server,
            RomeCommand::RunServer { .. } => false,
            RomeCommand::PrintSocket => false,
        }
    }

    pub const fn has_metrics(&self) -> bool {
        false
    }
}

pub fn parse_command() -> OptionParser<RomeCommand> {
    rome_command()
        .header("Rome CLI")
        .usage("rome COMMAND [ARG]")
		.version(VERSION)
		.descr("Rome official CLI. Use it to check the health of your project or run ti to check single files!")
}
