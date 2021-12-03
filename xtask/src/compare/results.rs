use crate::coverage::files::{Outcome, TestResult, TestResults};
use ascii_table::{AsciiTable, Column};
use colored::Colorize;
use std::collections::HashSet;
use std::iter::FromIterator;
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
					std::cmp::Ordering::Equal => format!(""),
					std::cmp::Ordering::Greater => {
						if i_am_passed_results {
							format!("{}{}", good, up)
						} else {
							format!("{}{}", bad, up)
						}
					}
				}
			} else {
				format!("")
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

		fn summary(title: &str, tests: &HashSet<&TestResult>) {
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
				paths.sort();
				for path in paths {
					println!("{}", path);
				}
				println!("```");
				println!("</details>");
			}
		}

		summary(":fire: Regression", &report_diff.regression);
		summary(":tada: Fixed", &report_diff.newly_fixed);
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

struct ReportDiff<'a> {
	pub regression: HashSet<&'a TestResult>,
	pub newly_fixed: HashSet<&'a TestResult>,
	pub failed_to_panic: HashSet<&'a TestResult>,
	pub panic_to_failed: HashSet<&'a TestResult>,
	pub added_tests: HashSet<&'a TestResult>,
	pub removed_tests: HashSet<&'a TestResult>,
}

fn compare_diffs<'a>(
	base_results: &'a TestResults,
	new_results: &'a TestResults,
) -> ReportDiff<'a> {
	let (base_passed, base_not_passed): (Vec<&TestResult>, Vec<&TestResult>) = base_results
		.details
		.iter()
		.partition(|result| result.outcome == Outcome::Passed);
	let (new_passed, new_not_passed): (Vec<&TestResult>, Vec<&TestResult>) = new_results
		.details
		.iter()
		.partition(|result| result.outcome == Outcome::Passed);

	let base_all: HashSet<&TestResult> = HashSet::from_iter(base_results.details.iter());
	let base_passed: HashSet<&TestResult> = HashSet::from_iter(base_passed);
	let base_not_passed: HashSet<&TestResult> = HashSet::from_iter(base_not_passed);
	let base_failed: HashSet<&TestResult> = base_results
		.details
		.iter()
		.filter(|result| result.outcome == Outcome::Failed)
		.collect();
	let base_panicked: HashSet<&TestResult> = base_results
		.details
		.iter()
		.filter(|result| result.outcome == Outcome::Panicked)
		.collect();

	let new_all: HashSet<&TestResult> = HashSet::from_iter(new_results.details.iter());
	let new_passed: HashSet<&TestResult> = HashSet::from_iter(new_passed);
	let new_not_passed: HashSet<&TestResult> = HashSet::from_iter(new_not_passed);
	let new_failed: HashSet<&TestResult> = new_results
		.details
		.iter()
		.filter(|result| result.outcome == Outcome::Failed)
		.collect();
	let new_panicked: HashSet<&TestResult> = new_results
		.details
		.iter()
		.filter(|result| result.outcome == Outcome::Panicked)
		.collect();

	// Regression: passed on main, failing or panicking on the feature branch (bad)
	let regression = base_passed.intersection(&new_not_passed);

	// Fixed: panicked or failed on main, passing on the feature branch (good)
	let newly_fixed = base_not_passed.intersection(&new_passed);

	// FailedToPanic: failed on main, panicking on the feature branch (kind of bad)
	let failed_to_panic = base_failed.intersection(&new_panicked);

	// PanicToFailed: panicked on main, failing on the feature branch (kind of good?)
	let panic_to_failed = base_panicked.intersection(&new_failed);

	// New: Newly added tests
	let added_tests = new_all.difference(&base_all);

	// Removed: Removed tests
	let removed_tests = base_all.difference(&new_all);

	ReportDiff {
		regression: regression.copied().collect(),
		newly_fixed: newly_fixed.copied().collect(),
		failed_to_panic: failed_to_panic.copied().collect(),
		panic_to_failed: panic_to_failed.copied().collect(),
		added_tests: added_tests.copied().collect(),
		removed_tests: removed_tests.copied().collect(),
	}
}
