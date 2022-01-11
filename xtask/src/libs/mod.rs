use ansi_rgb::{red, Foreground};
use itertools::Itertools;
use rslint_errors::Diagnostic;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::time::Duration;
use std::{path::PathBuf, str::FromStr};

fn err_to_string<E: std::fmt::Debug>(e: E) -> String {
	format!("{:?}", e)
}

#[cfg(feature = "dhat-on")]
fn print_diff(before: dhat::Stats, current: dhat::Stats) -> dhat::Stats {
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

pub fn get_code(lib: &str) -> Result<(String, String), String> {
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
			Ok((filename.to_string(), code))
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

					std::fs::read_to_string(&file)
						.map_err(err_to_string)
						.map(|code| (filename.to_string(), code))
				}
				Err(e) => Err(format!("{:?}", e)),
			}
		}
	}
}

pub fn run(filter: String, criterion: bool) {
	let regex = regex::Regex::new(filter.as_str()).unwrap();
	let libs = include_str!("libs.txt").lines();

	for lib in libs {
		if !regex.is_match(lib) {
			continue;
		}

		let code = get_code(lib);

		match code {
			Ok((id, code)) => {
				let code = code.as_str();

				// Do all steps with criterion now
				if criterion {
					let mut criterion = criterion::Criterion::default().without_plots();
					criterion.bench_function(lib, |b| {
						b.iter(|| {
							let _ = criterion::black_box(rslint_parser::parse_module(code, 0));
						})
					});
				} else {
					//warmup
					rslint_parser::parse_module(code, 0);
				}

				let result = benchmark_lib(&id, code);
				println!("Benchmark: {}", lib);
				println!("{}", result);
			}
			Err(e) => println!("{:?}", e),
		}
	}
}

fn benchmark_lib(id: &str, code: &str) -> BenchmarkResult {
	#[cfg(feature = "dhat-on")]
	println!("Start");
	#[cfg(feature = "dhat-on")]
	let stats = dhat::get_stats().unwrap();

	let tokenizer_timer = timing::start();
	let (tokens, mut diagnostics) = rslint_parser::tokenize(code, 0);
	let tok_source = rslint_parser::TokenSource::new(code, &tokens);
	let tokenization_duration = tokenizer_timer.stop();

	#[cfg(feature = "dhat-on")]
	println!("Tokenizer");
	#[cfg(feature = "dhat-on")]
	let stats = print_diff(stats, dhat::get_stats().unwrap());

	let parser_timer = timing::start();
	let (events, parsing_diags, tokens) = {
		let mut parser =
			rslint_parser::Parser::new(tok_source, 0, rslint_parser::Syntax::default().module());
		rslint_parser::syntax::program::parse(&mut parser);
		let (events, parsing_diags) = parser.finish();
		(events, parsing_diags, tokens)
	};
	let parse_duration = parser_timer.stop();

	#[cfg(feature = "dhat-on")]
	println!("Parsed");
	#[cfg(feature = "dhat-on")]
	let stats = print_diff(stats, dhat::get_stats().unwrap());

	let tree_sink_timer = timing::start();
	let mut tree_sink = rslint_parser::LosslessTreeSink::new(code, &tokens);
	rslint_parser::process(&mut tree_sink, events, parsing_diags);
	let (_green, sink_diags) = tree_sink.finish();
	let tree_sink_duration = tree_sink_timer.stop();

	#[cfg(feature = "dhat-on")]
	println!("Tree-Sink");
	#[cfg(feature = "dhat-on")]
	print_diff(stats, dhat::get_stats().unwrap());

	diagnostics.extend(sink_diags);
	BenchmarkResult {
		id: id.to_string(),
		tokenization: tokenization_duration,
		parsing: parse_duration,
		tree_sink: tree_sink_duration,
		diagnostics,
	}
}

#[derive(Debug, Clone)]
struct BenchmarkResult {
	id: String,
	tokenization: Duration,
	parsing: Duration,
	tree_sink: Duration,
	diagnostics: Vec<Box<Diagnostic>>,
}

impl BenchmarkResult {
	fn total(&self) -> Duration {
		self.tokenization.add(self.parsing).add(self.tree_sink)
	}
}

impl Display for BenchmarkResult {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let _ = writeln!(f, "\tTokenization: {:>10?}", self.tokenization);
		let _ = writeln!(f, "\tParsing:      {:>10?}", self.parsing);
		let _ = writeln!(f, "\tTree_sink:    {:>10?}", self.tree_sink);
		let _ = writeln!(f, "\t              ----------");
		let _ = writeln!(f, "\tTotal:        {:>10?}", self.total());

		let _ = writeln!(
			f,
			"\t[{}] Total Time: {:?} (tokenization: {:?}, parsing: {:?}, tree_sink: {:?})",
			self.id,
			self.total(),
			self.tokenization,
			self.parsing,
			self.tree_sink,
		);

		let _ = writeln!(f, "\tDiagnostics");
		for (severity, items) in &self.diagnostics.iter().group_by(|x| x.severity) {
			let _ = writeln!(f, "\t\t{:?}: {}", severity, items.count());
		}

		Ok(())
	}
}
