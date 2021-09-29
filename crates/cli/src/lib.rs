use clap::{crate_version, App, AppSettings, Arg};
use rome_formatter::{format, FormatOptions, IndentStyle};
use std::{path::PathBuf, str::FromStr};

/// Main function to run Rome CLI
pub fn run_cli() {
	let matches = App::new("rome")
		.about("The official Rome CLI")
		.version(crate_version!())
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.subcommand(
			App::new("format")
				.about("Format a file")
				.arg(
					Arg::new("indent_style")
						.long("indent-style")
						.about("The style of indentation")
						.value_name("tab|space")
						.default_value("tab")
						.validator(|value| IndentStyle::from_str(value).map(|_| ())),
				)
				.arg(
					Arg::new("indent_size")
						.long("indent-size")
						.about("The size of the indent.")
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
						.about("File to format")
						.required(true)
						.validator(|value| {
							let path = PathBuf::from(&value);
							if !path.exists() {
								return Err(format!("The file \"{}\" doesn't exist.", value));
							}
							Ok(())
						}),
				),
		)
		.try_get_matches();
	let subcommand_matches = match &matches {
		Ok(r) => r.subcommand(),
		Err(err) => err.exit(),
	};

	match subcommand_matches {
		Some(("format", matches)) => {
			let size = matches.value_of("indent_size");
			let style = matches.value_of("indent_style");
			let input = matches.value_of("input").unwrap();
			let input = PathBuf::from(&input);
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

			format(input.as_path(), FormatOptions::new(options));
		}
		// Thanks to the settings AppSettings::SubcommandRequiredElseHelp we should not be there
		_ => clap::Error::with_description(
			"Sub command not found".to_string(),
			clap::ErrorKind::InvalidSubcommand,
		)
		.exit(),
	}
}
