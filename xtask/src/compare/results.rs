use crate::coverage::files::{Outcome, TestResults};
use ascii_table::{AsciiTable, Column};
use colored::Colorize;
use std::{fs::File, path::Path};

pub fn emit_compare(base: &Path, new: &Path, markdown: bool) {
	let base_results: TestResults =
		serde_json::from_reader(File::open(base).expect("Can't read the file of the base results"))
			.expect("Can't parse the JSON file of the base results");
	let new_results: TestResults =
		serde_json::from_reader(File::open(new).expect("Can't read the file of the new results"))
			.expect("Can't parse the JSON file of the new results");

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
		fn diff_format(diff: isize) -> String {
			format!(
				"{}{}{}{}",
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
			diff_format(total_diff)
		);

		println!(
			"| Passed | {} | {} | {} |",
			base_passed,
			new_passed,
			diff_format(passed_diff)
		);

		println!(
			"| Failed | {} | {} | {} |",
			base_failed,
			new_failed,
			diff_format(failed_diff)
		);

		println!(
			"| Panics | {} | {} | {} |",
			base_passed,
			new_passed,
			diff_format(panics_diff)
		);

		println!(
			"| Coverage | {:.2}% | {:.2}% | {} |",
			base_coverage,
			new_coverage,
			format!(
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

		if !report_diff.fixed.is_empty() {
			println!();
			println!(
				"<details><summary><b>Fixed tests ({}):</b></summary>",
				report_diff.fixed.len()
			);
			println!("\n```");
			for test in report_diff.fixed {
				println!("{}", test);
			}
			println!("```");
			println!("</details>");
		}

		if !report_diff.failed.is_empty() {
			println!();
			println!(
				"<details><summary><b>Failed tests ({}):</b></summary>",
				report_diff.failed.len()
			);
			println!("\n```");
			for test in report_diff.failed {
				println!("{}", test);
			}
			println!("```");
			println!("</details>");
		}

		if !report_diff.new_panics.is_empty() {
			println!();
			println!(
				"<details><summary><b>New panics ({}):</b></summary>",
				report_diff.new_panics.len()
			);
			println!("\n```");
			for test in report_diff.new_panics {
				println!("{}", test);
			}
			println!("```");
			println!("</details>");
		}

		if !report_diff.panic_fixed.is_empty() {
			println!();
			println!(
				"<details><summary><b>Panics fixed ({}):</b></summary>",
				report_diff.panic_fixed.len()
			);
			println!("\n```");
			for test in report_diff.panic_fixed {
				println!("{}", test);
			}
			println!("```");
			println!("</details>");
		}
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

		let passed_diff = &base_passed - &new_passed;
		let failed_diff = &base_failed - &new_failed;
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

struct ReportDiff<'a> {
	pub fixed: Vec<&'a str>,
	pub failed: Vec<&'a str>,
	pub new_panics: Vec<&'a str>,
	pub panic_fixed: Vec<&'a str>,
}

impl<'a> ReportDiff<'a> {
	pub fn new() -> Self {
		Self {
			fixed: vec![],
			failed: vec![],
			new_panics: vec![],
			panic_fixed: vec![],
		}
	}
}

fn compare_diffs<'a>(
	base_results: &'a TestResults,
	new_results: &'a TestResults,
) -> ReportDiff<'a> {
	let mut report_diff = ReportDiff::new();
	for base_result in &base_results.details {
		let test_to_analyze = new_results
			.details
			.iter()
			.find(|new_test| new_test.path.as_os_str().eq(base_result.path.as_os_str()));

		if let Some(test_to_analyze) = test_to_analyze {
			match (&base_result.outcome, &test_to_analyze.outcome) {
				// we want to ignore cases where both results yield the same enum
				// this means that their status hasn't changed, not worth tracking
				(b, n) if b == n => {}
				// the new result passed
				(_, Outcome::Passed) => report_diff
					.fixed
					.push(test_to_analyze.path.as_os_str().to_str().unwrap()),
				// an old test passed but now failed
				(Outcome::Passed, Outcome::Failed) => report_diff
					.failed
					.push(test_to_analyze.path.as_os_str().to_str().unwrap()),
				// an existing test now panics
				(_, Outcome::Panicked) => report_diff
					.new_panics
					.push(test_to_analyze.path.as_os_str().to_str().unwrap()),
				// a panic error is now fixed
				(Outcome::Panicked, _) => report_diff
					.panic_fixed
					.push(test_to_analyze.path.as_os_str().to_str().unwrap()),
				_ => {}
			}
		}
	}

	report_diff
}
