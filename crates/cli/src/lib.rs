use clap::{crate_version, App, Arg};
use rome_formatter::{format, FormatOptions, IndentStyle};
use std::path::PathBuf;

/// Main function to run Rome CLI
pub fn run_cli() {
	let result_matches = App::new("rome")
		.about("The official Rome CLI")
		.version(crate_version!())
		.subcommand(
			App::new("format")
				.about("Format a file")
				.arg(
					Arg::new("indentStyle")
						.long("indent-style")
						.about("The style of indentation")
						.value_name("tab|space")
						// .possible_values(&["tabs", "spaces"])
						.default_value("tab")
						.validator(|value| {
							if !value.is_empty() {
								if value.eq("tab") || value.eq("space") {
									return Ok(());
								}
								return Err(
									r#"Invalid indent-style value.  Only "tabs" and "spaces" are supported. "#,
								);
							}
							Ok(())
						}),
				)
				.arg(
					Arg::new("indentSize")
						.long("indent-size")
						.about("The size of the indent.")
						.value_name("NUMBER")
						.default_value("2")
						.validator(|value| {
							let number = value.parse::<u16>();
							match number {
								Ok(_) => Ok(()),
								Err(_) => Err("Invalid indent-size value. Try using a number"),
							}
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

	match result_matches {
		Ok(matches) => {
			if let Some(matches) = matches.subcommand_matches("format") {
				let size = matches.value_of("indentSize");
				let style = matches.value_of("indentStyle");
				let input = matches.value_of("input").unwrap();
				let input = PathBuf::from(&input);
				let options = match style {
					Some(style) => match style {
						"tab" => IndentStyle::Tab,
						"space" => {
							let size = size.unwrap_or("2");
							IndentStyle::Space(size.parse::<u8>().unwrap_or(2))
						}
						_ => IndentStyle::Tab,
					},
					None => IndentStyle::Tab,
				};
				format(input, FormatOptions::new(options));
			}
		}
		Err(err) => err.exit(),
	}
}
