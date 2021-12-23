use ansi_rgb::{red, Foreground};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::time::Duration;
use itertools::Itertools;
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

pub fn run(filter: String, criterion: bool) {
	let regex = regex::Regex::new(filter.as_str()).unwrap();
	let libs = include_str!("libs.txt").lines();

	for lib in libs {
		if !regex.is_match(lib) {
			continue;
		}

		let code = get_code(lib);

		match code {
			Ok(code) => {
				println!("Benchmark: {}", lib);
				let result = benchmark_lib(&code);
				println!("\tTokenization: {:>10?}", result.tokenization);
				println!("\tParsing:      {:>10?}", result.parsing);
				println!("\tTree_sink:    {:>10?}", result.tree_sink);
				println!("\t              ----------");
				println!("\tTotal:        {:>10?}", result.total());
				let text = code.as_str();
				
				// Do all steps with criterion now
				if criterion {
					let mut criterion = criterion::Criterion::default()
						.without_plots();
					criterion.bench_function(lib, |b| {
						b.iter(|| {
							let _ = rslint_parser::parse_module(text, 0);
						})
					});
				}
			}
			Err(e) => println!("{:?}", e),
		}
	}
}

fn benchmark_lib(code: &str) -> BenchmarkResult {
	#[cfg(feature = "dhat-on")]
	println!("Start");
	#[cfg(feature = "dhat-on")]
	let stats = dhat::get_stats().unwrap();

	let tokenizer_timer = timing::start();
	let (tokens, mut errors) = rslint_parser::tokenize(code, 0);
	let tok_source = rslint_parser::TokenSource::new(code, &tokens);
	let tokenization_duration = tokenizer_timer.stop();

	#[cfg(feature = "dhat-on")]
	println!("Tokenizer");
	#[cfg(feature = "dhat-on")]
	let stats = print_diff(stats, dhat::get_stats().unwrap());

	let parser_timer = timing::start();
	let (events, parse_errors, tokens) = {
		let mut parser =
			rslint_parser::Parser::new(tok_source, 0, rslint_parser::Syntax::default().module());
		rslint_parser::syntax::program::parse(&mut parser);
		let (events, p_errs) = parser.finish();
		(events, p_errs, tokens)
	};
	let parse_duration = parser_timer.stop();

	errors.extend(parse_errors);

	#[cfg(feature = "dhat-on")]
	println!("Parsed");
	#[cfg(feature = "dhat-on")]
	let stats = print_diff(stats, dhat::get_stats().unwrap());

	let tree_sink_timer = timing::start();
	let mut tree_sink = rslint_parser::LosslessTreeSink::new(code, &tokens);
	rslint_parser::process(&mut tree_sink, events, errors);
	let (_green, _parse_errors) = tree_sink.finish();

	#[cfg(feature = "dhat-on")]
	println!("Tree-Sink");
	#[cfg(feature = "dhat-on")]
	print_diff(stats, dhat::get_stats().unwrap());

	let tree_sink_duration = tree_sink_timer.stop();
	BenchmarkResult {
		tokenization: tokenization_duration,
		parsing: parse_duration,
		tree_sink: tree_sink_duration,
	}
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct BenchmarkResult {
	tokenization: Duration,
	parsing: Duration,
	tree_sink: Duration,
}

impl BenchmarkResult {
	fn total(&self) -> Duration {
		self.tokenization.add(self.parsing).add(self.tree_sink)
	}
}

impl PartialOrd for BenchmarkResult {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for BenchmarkResult {
	fn cmp(&self, other: &Self) -> Ordering {
		self.total().cmp(&other.total())
	}
}

impl Display for BenchmarkResult {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"total: {:?} (tokenization: {:?}, parsing: {:?}, tree_sink: {:?})",
			self.total(),
			&self.tokenization,
			&self.parsing,
			&self.tree_sink,
		)
	}
}
