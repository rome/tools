use ansi_rgb::{red, Foreground};
use std::{path::PathBuf, str::FromStr};

fn err_to_string<E: std::fmt::Debug>(e: E) -> String {
	format!("{:?}", e)
}

pub fn get_code(lib: &str) -> Result<String, String> {
	let url = url::Url::from_str(lib).map_err(err_to_string)?;
	let segments = url
		.path_segments()
		.ok_or_else(|| "lib url has no segments".to_string())?;
	let filename = segments
		.last()
		.ok_or_else(|| "lib url has no segments".to_string())?;

	let mut file = PathBuf::from_str("target").map_err(err_to_string)?;
	file.push(filename);

	match std::fs::read_to_string(&file) {
		Ok(code) => {
			println!("[{}] - using [{}]", filename.fg(red()), file.display());
			Ok(code)
		}
		Err(_) => {
			println!(
				"[{}] - Downloading [{}] to [{}]",
				filename,
				lib,
				file.display()
			);
			match ureq::get(lib).call() {
				Ok(response) => {
					let mut reader = response.into_reader();

					let _ = std::fs::remove_file(&file);
					let mut writer = std::fs::File::create(&file).map_err(err_to_string)?;
					let _ = std::io::copy(&mut reader, &mut writer);

					std::fs::read_to_string(&file).map_err(err_to_string)
				}
				Err(e) => Err(format!("{:?}", e)),
			}
		}
	}
}

pub fn run(filter: String) {
	let regex = regex::Regex::new(filter.as_str()).unwrap();
	let libs = include_str!("libs.txt").lines();
	for lib in libs {
		if !regex.is_match(lib) {
			continue;
		}

		let code = get_code(lib);
		match code {
			Ok(code) => {
				let t = timing::start();
				let _ = std::panic::catch_unwind(|| {
					let text = code;

					let tokenizer_timing = timing::start();
					let (tokens, mut errors) = rslint_parser::tokenize(text.as_str(), 0);
					let tok_source = rslint_parser::TokenSource::new(text.as_str(), &tokens);
					println!("\ttokenizer took {:?}", tokenizer_timing.stop());

					let parser_timing = timing::start();
					let (events, errors, tokens) = {
						let mut parser = rslint_parser::Parser::new(
							tok_source,
							0,
							rslint_parser::Syntax::default().module(),
						);
						rslint_parser::syntax::program::parse(&mut parser);
						let (events, p_errs) = parser.finish();
						errors.extend(p_errs);
						(events, errors, tokens)
					};
					println!("\tparser took {:?}", parser_timing.stop());

					let treesink_timing = timing::start();
					let mut tree_sink =
						rslint_parser::LosslessTreeSink::new(text.as_str(), &tokens);
					rslint_parser::process(&mut tree_sink, events, errors);
					let (_green, _parse_errors) = tree_sink.finish();
					println!("\ttree sink took {:?}", treesink_timing.stop());
				});
				let dur = t.stop();
				println!("total: {:?}", dur);
			}
			Err(e) => println!("{:?}", e),
		}
	}

	println!("end");
}
