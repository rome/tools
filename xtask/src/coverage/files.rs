use colored::Colorize;
use indicatif::ProgressBar;
use regex::Regex;
use rslint_parser::ParserError;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fs::read_to_string;
use std::io;
use std::path::PathBuf;
use std::{fs::File, io::Write};
use walkdir::WalkDir;
use yastl::Pool;

use crate::{project_root, BASE_RESULT_FILE};

const BASE_PATH: &str = "xtask/src/coverage/test262/test";

#[derive(Debug, Clone)]
pub struct TestFile {
	pub meta: MetaData,
	pub code: String,
	pub path: PathBuf,
}

/// Representation of the YAML metadata in Test262 tests.
// taken from the boa project
#[derive(Debug, Clone, Deserialize)]
pub struct MetaData {
	pub description: Box<str>,
	pub esid: Option<Box<str>>,
	pub es5id: Option<Box<str>>,
	pub es6id: Option<Box<str>>,
	#[serde(default)]
	pub info: Box<str>,
	#[serde(default)]
	pub features: Box<[Box<str>]>,
	#[serde(default)]
	pub includes: Box<[Box<str>]>,
	#[serde(default)]
	pub flags: Box<[TestFlag]>,
	#[serde(default)]
	pub negative: Option<Negative>,
	#[serde(default)]
	pub locale: Box<[Box<str>]>,
}

/// Negative test information structure.
#[derive(Debug, Clone, Deserialize)]
pub struct Negative {
	pub phase: Phase,
	#[serde(rename = "type")]
	pub error_type: Box<str>,
}

/// Individual test flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TestFlag {
	OnlyStrict,
	NoStrict,
	Module,
	Raw,
	Async,
	Generated,
	#[serde(rename = "CanBlockIsFalse")]
	CanBlockIsFalse,
	#[serde(rename = "CanBlockIsTrue")]
	CanBlockIsTrue,
	#[serde(rename = "non-deterministic")]
	NonDeterministic,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
	Parse,
	Early,
	Resolution,
	Runtime,
}

fn read_metadata(code: &str) -> io::Result<MetaData> {
	use once_cell::sync::Lazy;

	/// Regular expression to retrieve the metadata of a test.
	static META_REGEX: Lazy<Regex> = Lazy::new(|| {
		Regex::new(r#"/\*\-{3}((?:.|\n)*)\-{3}\*/"#)
			.expect("could not compile metadata regular expression")
	});

	let yaml = META_REGEX
		.captures(code)
		.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "no metadata found"))?
		.get(1)
		.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "no metadata found"))?
		.as_str();

	serde_yaml::from_str(yaml).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn get_test_files(query: Option<&str>, pool: &Pool) -> Vec<TestFile> {
	let start = std::time::Instant::now();

	let files = WalkDir::new(BASE_PATH)
		.into_iter()
		.filter_map(Result::ok)
		.filter(|file| {
			if let Some(query) = query {
				file.path()
					.to_str()
					.map_or(true, |path| path.contains(query))
			} else {
				true
			}
		})
		.collect::<Vec<_>>();

	let pb = ProgressBar::new(files.len() as u64);
	pb.set_message(&format!("{} test files", "Loading".bold().cyan()));
	pb.set_style(super::default_bar_style());

	let (tx, rx) = std::sync::mpsc::channel();

	pool.scoped(|scope| {
		let pb = &pb;
		for file in files {
			let tx = tx.clone();

			scope.execute(move || {
				fn parse_file(entry: walkdir::DirEntry) -> Option<TestFile> {
					let code = read_to_string(entry.path()).ok()?;
					let meta = read_metadata(&code).ok()?;
					let path = entry.into_path();
					Some(TestFile { meta, code, path }).filter(|file| file.meta.features.is_empty())
				}

				if let Some(file) = parse_file(file) {
					tx.send(file).unwrap();
				}

				pb.inc(1);
			});
		}
	});
	drop(tx);
	let files = rx.into_iter().collect();

	pb.finish_and_clear();
	println!(
		"{} test files in {:.2}s",
		"Loaded".bold().bright_green(),
		start.elapsed().as_secs_f32()
	);

	files
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResult {
	#[serde(skip)]
	pub fail: Option<FailReason>,
	#[serde(rename = "o")]
	pub outcome: Outcome,
	#[serde(rename = "h")]
	pub path: PathBuf,
	#[serde(skip)]
	pub code: String,
}

#[derive(Debug)]
pub enum FailReason {
	IncorrectlyPassed,
	IncorrectlyErrored(Vec<ParserError>),
	ParserPanic(Box<dyn Any + Send + 'static>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Outcome {
	Passed,
	Failed,
	Panicked,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResults {
	#[serde(rename = "s")]
	pub summary: Summary,
	#[serde(rename = "p")]
	pub details: Vec<TestResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
	#[serde(rename = "a")]
	pub tests_ran: u32,
	#[serde(rename = "pa")]
	pub passed: u32,
	#[serde(rename = "f")]
	pub failed: u32,
	#[serde(rename = "pc")]
	pub panics: u32,
	#[serde(rename = "c")]
	pub coverage: f64,
}

impl Default for TestResults {
	fn default() -> Self {
		Self {
			summary: Summary {
				tests_ran: 0,
				passed: 0,
				failed: 0,
				panics: 0,
				coverage: 0.0,
			},
			details: vec![],
		}
	}
}

impl TestResults {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn store_results(&mut self, results: Vec<TestResult>) {
		self.details = results;
		let passed = self.passed_tests() as u32;
		let tests_ran = self.details.len();
		let coverage = (passed as f64 / tests_ran as f64) * 100.0;
		self.summary = Summary {
			tests_ran: self.details.len() as u32,
			passed,
			failed: self.errored_tests() as u32,
			panics: self.panicked_tests() as u32,
			coverage,
		};
	}

	pub fn panicked_tests(&self) -> usize {
		self.details
			.iter()
			.filter(|res| matches!(res.fail, Some(FailReason::ParserPanic(_))))
			.count()
	}

	pub fn errored_tests(&self) -> usize {
		self.details
			.iter()
			.filter(|res| {
				matches!(
					res.fail,
					Some(FailReason::IncorrectlyErrored(_)) | Some(FailReason::IncorrectlyPassed)
				)
			})
			.count()
	}

	pub fn passed_tests(&self) -> usize {
		self.details.iter().filter(|res| res.fail.is_none()).count()
	}

	/// Saves results into a JSON file inside the temporary directory of the OS
	pub fn dump_to_json(&self) {
		let json = serde_json::to_string(&self).unwrap();
		let path = project_root().join(BASE_RESULT_FILE);

		let mut file = File::create(&path).expect("Can't open the JSON file");

		file.write_all(json.as_bytes())
			.expect("Can't write in the JSON file");
		println!();
		println!("The test result report has been saved in: {:?}", &path);
	}
}
