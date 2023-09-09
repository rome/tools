use crate::vcs::create_vcs_client;
use crate::CliDiagnostic;
use bpaf::Bpaf;
use rome_console::markup;
use rome_service::Configuration;
use std::str::FromStr;

/// Global options applied to all commands
#[derive(Debug, Clone, Bpaf)]
pub struct CliOptions {
    /// Set the formatting mode for markup: "off" prints everything as plain text, "force" forces the formatting of markup using ANSI even if the console output is determined to be incompatible
    #[bpaf(long("colors"), argument("off|force"))]
    pub colors: Option<ColorsArg>,

    /// Connect to a running instance of the Rome daemon server.
    #[bpaf(long("use-server"), switch, fallback(false))]
    pub use_server: bool,

    /// Print additional verbose advices on diagnostics
    #[bpaf(long("verbose"), switch, fallback(false))]
    pub verbose: bool,

    /// Set the filesystem path to the directory of the rome.json configuration file
    #[bpaf(long("config-path"), argument("PATH"), optional)]
    pub config_path: Option<String>,

    /// Cap the amount of diagnostics displayed.
    #[bpaf(
        long("max-diagnostics"),
        argument("NUMBER"),
        fallback(20),
        display_fallback
    )]
    pub max_diagnostics: u16,

    /// Skip over files containing syntax errors instead of emitting an error diagnostic.
    #[bpaf(long("skip-errors"), switch)]
    pub skip_errors: bool,

    /// Silence errors that would be emitted in case no files were processed during the execution of the command.
    #[bpaf(long("no-errors-on-unmatched"), switch)]
    pub no_errors_on_unmatched: bool,

    /// Tell Rome to exit with an error code if some diagnostics emit warnings.
    #[bpaf(long("error-on-warnings"), switch)]
    pub error_on_warnings: bool,

    /// Reports information using the JSON format
    #[bpaf(long("json"), switch, hide_usage)]
    pub json: bool,

    /// Only runs the operation on files that have changed. VCS support needs to be enabled. Doesn't work when running `rome ci`.
    #[bpaf(long("changed"), switch)]
    pub changed: bool,
}

impl CliOptions {
    /// It validates the current CLI options against. If not errors are found, it calls the VCS client
    /// via [std::process::Command] and returns a list of changed files
    ///
    /// ## Errors
    /// - `--changed` is passed and the VCS support is disabled
    /// - errors are raised when calling the VCS client via [std::process::Command]
    pub(crate) fn compute_changed_files(
        &self,
        configuration: &Configuration,
    ) -> Result<Vec<String>, CliDiagnostic> {
        if let Some(vcs) = configuration.vcs.as_ref() {
            if vcs.is_disabled() && self.changed {
                return Err(CliDiagnostic::incompatible_end_configuration(markup! {
                    "You provided the "<Emphasis>"--changed"</Emphasis>" argument, but you haven't enabled VCS support. This is an error."
                }));
            }
            if let Some(client_kind) = vcs.client_kind.as_ref() {
                let vcs_client = create_vcs_client(client_kind);
                return vcs_client.changed_files();
            }
        }

        Ok(vec![])
    }
}

#[derive(Debug, Clone)]
pub enum ColorsArg {
    Off,
    Force,
}

impl FromStr for ColorsArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(Self::Off),
            "force" => Ok(Self::Force),
            _ => Err(format!(
                "value {s:?} is not valid for the --colors argument"
            )),
        }
    }
}
