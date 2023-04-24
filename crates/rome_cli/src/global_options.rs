use bpaf::Bpaf;
use rome_diagnostics::MAXIMUM_DISPLAYABLE_DIAGNOSTICS;
use std::str::FromStr;

#[derive(Debug, Clone, Bpaf)]
pub(crate) struct GlobalOptions {
    /// Set the formatting mode for markup: "off" prints everything as plain text, "force" forces the formatting of markup using ANSI even if the console output is determined to be incompatible
    #[bpaf(long("colors"), argument("off|force"))]
    colors: Option<ColorsArg>,

    /// Connect to a running instance of the Rome daemon server.
    #[bpaf(long("use-server"), switch)]
    use_server: bool,

    /// Print additional verbose advices on diagnostics
    #[bpaf(long("verbose"), switch)]
    verbose: bool,

    /// Set the filesystem path to the directory of the rome.json configuration file
    #[bpaf(long("config-path"), argument("PATH"), optional)]
    config_path: Option<String>,

    /// Cap the amount of diagnostics displayed (default: 20)
    #[bpaf(
        long("max-diagnostics"),
        argument("NUMBER"),
        guard(
            check_max_diagnostics,
            "The value of the argument is too high, maximum accepted: 500"
        ),
        fallback(20)
    )]
    max_diagnostics: u16,
}

#[derive(Debug, Clone)]
pub(crate) enum ColorsArg {
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

fn check_max_diagnostics(number: &u16) -> bool {
    *number > MAXIMUM_DISPLAYABLE_DIAGNOSTICS
}
