use bpaf::Bpaf;
use std::str::FromStr;

#[derive(Debug, Clone, Bpaf)]
pub(crate) struct GlobalOptions {
    /// Set the formatting mode for markup: "off" prints everything as plain text, "force" forces the formatting of markup using ANSI even if the console output is determined to be incompatible
    #[bpaf(long("colors"), argument("off|force"))]
    colors: Option<ColorsArg>,

    /// Connect to a running instance of the Rome daemon server.
    #[bpaf(long("use-server"), switch)]
    use_server: Option<bool>,

    /// Print additional verbose advices on diagnostics
    #[bpaf(long("verbose"), switch)]
    verbose: Option<bool>,

    /// Set the filesystem path to the directory of the rome.json configuration file
    #[bpaf(long("config-path"), argument("PATH"))]
    config_path: Option<String>,
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
