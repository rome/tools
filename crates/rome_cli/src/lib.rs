use clap::{crate_version, App as ClapApp, AppSettings, Arg};
use rome_core::App;
use rome_formatter::IndentStyle;
use std::{path::PathBuf, str::FromStr};

/// Main function to run Rome CLI
pub fn run_cli() {
    let cli_app = ClapApp::new("rome")
        .about("The official Rome CLI")
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            ClapApp::new("format")
                .about("Format a file")
                .arg(
                    Arg::new("indent_style")
                        .long("indent-style")
                        .help("The style of indentation")
                        .value_name("tab|space")
                        .default_value("tab")
                        .validator(|value| IndentStyle::from_str(value).map(|_| ())),
                )
                .arg(
                    Arg::new("indent_size")
                        .long("indent-size")
                        .help("The size of the indent.")
                        .value_name("NUMBER")
                        .default_value("2")
                        .validator(|value| {
                            value
                                .parse::<u8>()
                                .map_err(|_| "Invalid indent-size value. Try using a number")
                        }),
                )
                .arg(
                    Arg::new("input")
                        .help("File to format")
                        .required(true)
                        .validator(|value| {
                            let path = PathBuf::from(&value);
                            if !path.exists() {
                                return Err(format!("The file \"{}\" doesn't exist.", value));
                            }
                            Ok(())
                        }),
                ),
        );

    let rome_app = App::new();

    match cli_app.get_matches().subcommand().unwrap() {
        ("format", matches) => {
            let size = matches.value_of("indent_size");
            let style = matches.value_of("indent_style");
            let input = matches.value_of("input").unwrap();
            let options: IndentStyle = style
                .map(|s| match s {
                    "tab" => IndentStyle::Tab,
                    "space" => {
                        let size = size.unwrap_or("2");
                        IndentStyle::Space(size.parse::<u8>().unwrap_or(2))
                    }
                    _ => IndentStyle::default(),
                })
                .unwrap_or_default();

            //format_file_and_save(&mut file, FormatOptions::new(options), &rome_app);
        }
        _ => unreachable!("clap should ensure we don't get here"),
    }
}
