use crate::coverage::files::{Outcome, TestResult, TestResults};
use ascii_table::{AsciiTable, Column};
use colored::Colorize;
use std::char::decode_utf16;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn emit_compare(base: &Path, new: &Path, markdown: bool) {
	let base_results = read_test_results(base, "base");
	let new_results = read_test_results(new, "new");

	let base_total = base_results.summary.tests_ran as isize;
	let new_total = new_results.summary.tests_ran as isize;
	let total_diff = new_total - base_total;

	let base_passed = base_results.summary.passed as isize;
	let new_passed = new_results.summary.passed as isize;
	let passed_diff = new_passed - base_passed;

	let base_failed = base_results.summary.failed as isize;
	let new_failed = new_results.summary.failed as isize;
	let failed_diff = new_failed - base_failed;

	let base_panics = base_results.summary.panics as isize;
	let new_panics = new_results.summary.panics as isize;
	let panics_diff = new_panics - base_panics;

	let base_coverage = base_results.summary.coverage;
	let new_coverage = new_results.summary.coverage;
	let coverage_diff = new_coverage - base_coverage;

	let report_diff = compare_diffs(&base_results, &new_results);

	if markdown {
		/// Generates a proper diff format, with some bold text if things change.
		fn diff_format(diff: isize, i_am_passed_results: bool, show_increase: bool) -> String {
			let good = "✅ ";
			let bad = "❌ ";
			let up = "⏫ ";
			let down = "⏬ ";

			let emoji = if show_increase {
				match diff.cmp(&0) {
					std::cmp::Ordering::Less => {
						if i_am_passed_results {
							format!("{}{}", bad, down)
						} else {
							format!("{}{}", good, down)
						}
					}
					std::cmp::Ordering::Equal => String::new(),
					std::cmp::Ordering::Greater => {
						if i_am_passed_results {
							format!("{}{}", good, up)
						} else {
							format!("{}{}", bad, up)
						}
					}
				}
			} else {
				String::new()
			};

			format!(
				"{}{}{}{}{}",
				emoji,
				if diff != 0 { "**" } else { "" },
				if diff > 0 { "+" } else { "" },
				diff,
				if diff != 0 { "**" } else { "" }
			)
		}

		println!("| Test result | `main` count | This PR count | Difference |");
		println!("| :---------: | :----------: | :-----------: | :--------: |");

		println!(
			"| Total | {} | {} | {} |",
			base_total,
			new_total,
			diff_format(total_diff, false, false)
		);

		println!(
			"| Passed | {} | {} | {} |",
			base_passed,
			new_passed,
			diff_format(passed_diff, true, true)
		);

		println!(
			"| Failed | {} | {} | {} |",
			base_failed,
			new_failed,
			diff_format(failed_diff, false, true)
		);

		println!(
			"| Panics | {} | {} | {} |",
			base_panics,
			new_panics,
			diff_format(panics_diff, false, true)
		);

		println!(
			"| Coverage | {:.2}% | {:.2}% | {} |",
			base_coverage,
			new_coverage,
			format_args!(
				"{}{}{:.2}%{}",
				if coverage_diff.abs() > f64::EPSILON {
					"**"
				} else {
					""
				},
				if coverage_diff > 0_f64 { "+" } else { "" },
				coverage_diff,
				if coverage_diff.abs() > f64::EPSILON {
					"**"
				} else {
					""
				},
			),
		);

		fn summary(title: &str, tests: &[&TestResult]) {
			if !tests.is_empty() {
				println!();
				println!(
					"<details><summary><b>{} ({}):</b></summary>",
					title,
					tests.len()
				);
				println!("\n```");
				let mut paths = tests
					.iter()
					.map(|test| test.path.as_os_str().to_str().unwrap())
					.collect::<Vec<&str>>();
				paths.sort_unstable();
				for path in paths {
					println!("{}", path);
				}
				println!("```");
				println!("</details>");
			}
		}

		summary(":fire: Regression", &report_diff.regression);
		summary(":tada: Fixed", &report_diff.fixed);
		summary(":boom: Failed to Panic", &report_diff.failed_to_panic);
		summary(
			":interrobang: Panic To Failed",
			&report_diff.panic_to_failed,
		);
		summary(":heavy_plus_sign: Added Tests", &report_diff.added_tests);
		summary(
			":heavy_minus_sign: Removed Tests",
			&report_diff.removed_tests,
		);
	} else {
		let mut table = AsciiTable::default();
		let mut counter = 0usize;

		let mut create_column = |name: colored::ColoredString| {
			let column = Column {
				header: name.to_string(),
				align: ascii_table::Align::Center,
				..Column::default()
			};
			table.columns.insert(counter, column);
			counter += 1;
		};

		println!("Test262 conformance changes:");

		create_column("Tests result".into());
		create_column("main branch".green());
		create_column("PR".yellow());
		create_column("Difference".cyan());

		let passed_diff = base_passed - new_passed;
		let failed_diff = base_failed - new_failed;
		let panics_diff = base_panics - new_panics;
		let passed_row: Vec<&dyn std::fmt::Display> =
			vec![&"Passed", &base_passed, &new_passed, &passed_diff];
		let failed_row: Vec<&dyn std::fmt::Display> =
			vec![&"Failed", &base_failed, &new_failed, &failed_diff];
		let panics_row: Vec<&dyn std::fmt::Display> =
			vec![&"Panics", &base_panics, &new_panics, &panics_diff];
		table.print(vec![passed_row, failed_row, panics_row]);
	}
}

fn read_test_results(path: &Path, name: &'static str) -> TestResults {
	let mut file = File::open(path)
		.unwrap_or_else(|err| panic!("Can't read the file of the {} results: {:?}", name, err));

	let mut buffer = Vec::new();
	file.read_to_end(&mut buffer)
		.unwrap_or_else(|err| panic!("Can't read the file of the {} results: {:?}", name, err));

	enum FileEncoding {
		Unknown,
		Utf8,
		Utf16Le,
		Utf16Be,
	}

	let mut encoding = FileEncoding::Unknown;
	let mut content: &[u8] = &buffer;

	// Read the BOM if present and skip it
	let bom = content.get(0..3);
	if let Some(&[0xef, 0xbb, 0xbf]) = bom {
		content = &content[3..];
		encoding = FileEncoding::Utf8;
	} else if let Some(&[0xfe, 0xff, _]) = bom {
		content = &content[2..];
		encoding = FileEncoding::Utf16Be;
	} else if let Some(&[0xff, 0xfe, _]) = bom {
		content = &content[2..];
		encoding = FileEncoding::Utf16Le;
	}

	if matches!(encoding, FileEncoding::Unknown | FileEncoding::Utf8) {
		// Attempt to parse as UTF-8
		let result = serde_json::from_slice(content);

		if let FileEncoding::Utf8 = encoding {
			// If the file is known to be UTF-8 unwrap the result
			return result.unwrap_or_else(|err| {
				panic!(
					"Can't parse the JSON file of the {} results: {:?}",
					name, err
				)
			});
		} else if let Ok(result) = result {
			// Otherwise only return if the parsing was successful
			return result;
		}
	}

	// If a UTF-16 BOM was found or an error was encountered, attempt to parse as UTF-16
	let content_str = decode_utf16(content.chunks(2).map(|bytes| match encoding {
		FileEncoding::Utf16Be => u16::from_be_bytes([bytes[0], bytes[1]]),
		FileEncoding::Utf16Le => u16::from_le_bytes([bytes[0], bytes[1]]),
		// If the encoding is unknown attempt to decode in native endianness
		FileEncoding::Unknown => u16::from_ne_bytes([bytes[0], bytes[1]]),
		FileEncoding::Utf8 => unreachable!(),
	}))
	.collect::<Result<String, _>>()
	.unwrap_or_else(|err| panic!("Can't read the file of the {} results: {:?}", name, err));

	serde_json::from_str(&content_str).unwrap_or_else(|err| {
		panic!(
			"Can't parse the JSON file of the {} results: {:?}",
			name, err
		)
	})
}

struct ReportDiff<'a> {
	pub regression: Vec<&'a TestResult>,
	pub fixed: Vec<&'a TestResult>,
	pub failed_to_panic: Vec<&'a TestResult>,
	pub panic_to_failed: Vec<&'a TestResult>,
	pub added_tests: Vec<&'a TestResult>,
	pub removed_tests: Vec<&'a TestResult>,
}

impl<'a> ReportDiff<'a> {
	pub fn new() -> Self {
		Self {
			regression: vec![],
			fixed: vec![],
			failed_to_panic: vec![],
			panic_to_failed: vec![],
			added_tests: vec![],
			removed_tests: vec![],
		}
	}
}

fn compare_diffs<'a>(
	base_results: &'a TestResults,
	new_results: &'a TestResults,
) -> ReportDiff<'a> {
	let mut report_diff = ReportDiff::new();

	let mut all_paths: HashSet<&PathBuf> = HashSet::new();

	let mut base_by_path: HashMap<&PathBuf, &TestResult> = HashMap::new();
	for detail in base_results.details.iter() {
		all_paths.insert(&detail.path);
		base_by_path.insert(&detail.path, detail);
	}

	let mut new_by_path: HashMap<&PathBuf, &TestResult> = HashMap::new();
	for detail in new_results.details.iter() {
		all_paths.insert(&detail.path);
		new_by_path.insert(&detail.path, detail);
	}

	for path in all_paths {
		let base_result = base_by_path.get(path);
		let new_result = new_by_path.get(path);

		match (base_result, new_result) {
			(None, Some(new)) => {
				report_diff.added_tests.push(new);
			}
			(Some(base), None) => {
				report_diff.removed_tests.push(base);
			}
			(Some(base), Some(new)) => {
				match (&base.outcome, &new.outcome) {
					(Outcome::Passed, Outcome::Failed | Outcome::Panicked) => {
						report_diff.regression.push(new)
					}
					(Outcome::Failed | Outcome::Panicked, Outcome::Passed) => {
						report_diff.fixed.push(new)
					}
					(Outcome::Failed, Outcome::Panicked) => report_diff.failed_to_panic.push(new),
					(Outcome::Panicked, Outcome::Failed) => report_diff.panic_to_failed.push(new),
					// we want to ignore cases where both results yield the same enum
					// this means that their status hasn't changed, not worth tracking
					_ => {}
				}
			}
			_ => unreachable!(),
		}
	}

	report_diff
}
