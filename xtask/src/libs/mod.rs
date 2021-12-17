use ansi_rgb::{red, Foreground};
use std::{path::PathBuf, str::FromStr};

fn err_to_string<E: std::fmt::Debug>(e: E) -> String {
	format!("{:?}", e)
}

#[cfg(feature = "dhat-on")]
fn print_diff(before: dhat::Stats, current: dhat::Stats) -> dhat::Stats {
	use dhat::HeapStats;
	use humansize::{file_size_opts as options, FileSize};

	println!("\tMemory");
	if let Some(heap) = &current.heap {
		println!("\t\tCurrent Blocks: {}", heap.curr_blocks);
		println!(
			"\t\tCurrent Bytes: {}",
			heap.curr_bytes.file_size(options::CONVENTIONAL).unwrap()
		);
		println!("\t\tMax Blocks: {}", heap.max_blocks);
		println!(
			"\t\tMax Bytes: {}",
			heap.max_bytes.file_size(options::CONVENTIONAL).unwrap()
		);
	}

	println!(
		"\t\tTotal Blocks: {}",
		current.total_blocks - before.total_blocks
	);
	println!(
		"\t\tTotal Bytes: {}",
		(current.total_bytes - before.total_bytes)
			.file_size(options::CONVENTIONAL)
			.unwrap()
	);

	current
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

					// Tokenizer
					// println!("Tokenizer");
					#[cfg(feature = "dhat-on")]
					#[cfg(feature = "dhat-on")]
					let stats = dhat::get_stats().unwrap();
					let tokenizer_timing = timing::start();
					let (tokens, mut errors) = rslint_parser::tokenize(text.as_str(), 0);
					let tok_source = rslint_parser::TokenSource::new(text.as_str(), &tokens);
					// println!("\tTime");
					// println!("\t\ttook {:?}", tokenizer_timing.stop());
					#[cfg(feature = "dhat-on")]
					let stats = print_diff(stats, dhat::get_stats().unwrap());

					// Parser
					// println!("Parser");
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
					// println!("\tTime");
					// println!("\t\ttook {:?}", parser_timing.stop());
					#[cfg(feature = "dhat-on")]
					let stats = print_diff(stats, dhat::get_stats().unwrap());

					// TreeSink
					// println!("TreeSink");
					// let treesink_timing = timing::start();
					// let mut tree_sink =
					// 	rslint_parser::LosslessTreeSink::new(text.as_str(), &tokens);
					// rslint_parser::process(&mut tree_sink, events, errors);
					// let (_green, _parse_errors) = tree_sink.finish();
					// println!("\tTime");
					// println!("\t\ttook {:?}", treesink_timing.stop());
					#[cfg(feature = "dhat-on")]
					let stats = print_diff(stats, dhat::get_stats().unwrap());
				});
				let dur = t.stop();
				println!("Total Time: {:?}", dur);
			}
			Err(e) => println!("{:?}", e),
		}
	}

	println!("end");
}
