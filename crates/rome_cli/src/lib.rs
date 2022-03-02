use pico_args::Arguments;
use rome_core::App;
use rome_formatter::{format_file_and_save, FormatOptions, IndentStyle};
use rome_path::RomePath;
use std::ffi::OsString;

const HELP: &str = concat!(
    "Rome CLI v",
    env!("CARGO_PKG_VERSION"),
    "
Available commands:
- format
- help
",
);

/// Main function to run Rome CLI
pub fn run_cli(args: Vec<OsString>) {
    let mut args = Arguments::from_vec(args);
    let subcommand = args.subcommand();
    match subcommand.as_ref().map(Option::as_deref) {
        Ok(Some("format")) => {
            let mut options = FormatOptions::default();

            let size = args
                .opt_value_from_str("--indent-size")
                .expect("failed to parse indent-size argument");

            let style = args
                .opt_value_from_str("--indent-style")
                .expect("failed to parse indent-style argument");

            match style {
                Some(IndentStyle::Tab) => {
                    options.indent_style = IndentStyle::Tab;
                }
                Some(IndentStyle::Space(default_size)) => {
                    options.indent_style = IndentStyle::Space(size.unwrap_or(default_size));
                }
                None => {}
            }

            let rome_app = App::new();
            for input in args.finish() {
                let mut file = RomePath::new(input);
                format_file_and_save(&mut file, options, &rome_app);
            }
        }
        Ok(None | Some("help")) => {
            println!("{HELP}");
        }
        Ok(Some(cmd)) => {
            panic!("unknown command {cmd:?}")
        }
        Err(err) => {
            panic!("failed to parse command: {err}")
        }
    }
}
