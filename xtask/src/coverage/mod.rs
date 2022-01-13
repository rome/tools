pub mod files;

use ascii_table::{AsciiTable, Column};
use colored::Colorize;
use files::*;
use rslint_parser::{parse_module, parse_text, ParserError};
use std::any::Any;
use std::path::PathBuf;
use yastl::Pool;

pub const TEST_JSON_PATH: &str = "xtask/src/base_results.json";

<<<<<<< HEAD
pub fn run(query: Option<&str>, pool: Pool, json: bool) {
    let files = get_test_files(query, &pool, json);
    let num_ran = files.len();

    let detailed = num_ran < 10;

    let pb = indicatif::ProgressBar::new(num_ran as u64);
    pb.set_position(1);
    pb.set_message(format!("{} tests", "Running".bold().cyan()));
    pb.set_style(default_bar_style());

    std::panic::set_hook(Box::new(|_| {}));
    let start_tests = std::time::Instant::now();
    let mut test_results = TestResults::new();

    let (tx, rx) = std::sync::mpsc::channel();

    pool.scoped(|scope| {
        let pb = &pb;
        for file in files {
            let tx = tx.clone();

            scope.execute(move || {
                let res = run_test_file(file);
                pb.inc(1);

                if detailed && res.fail.is_some() {
                    report_detailed_test(pb, &res);
                    tx.send(res).unwrap();
                    return;
                }

                if let Some(ref fail) = res.fail {
                    let reason = match fail {
                        FailReason::IncorrectlyPassed => "incorrectly passed parsing",
                        FailReason::IncorrectlyErrored(_) => "incorrectly threw an error",
                        FailReason::ParserPanic(_) => "panicked while parsing",
                    };
                    let msg = format!(
                        "{} '{}' {}",
                        "Test".bold().red(),
                        res.path
                            .strip_prefix("xtask/src/coverage/test262/test/")
                            .unwrap_or(&res.path)
                            .display(),
                        reason.bold()
                    );
                    pb.println(msg);
                }

                tx.send(res).unwrap();
            });
        }
    });
    drop(tx);

    test_results.store_results(rx.into_iter().collect::<Vec<_>>());

    let _ = std::panic::take_hook();

    pb.finish_and_clear();

    if json {
        test_results.dump_to_json();
    } else {
        println!(
            "\n{} {} tests in {:.2}s\n",
            "Ran".bold().bright_green(),
            num_ran,
            start_tests.elapsed().as_secs_f32()
        );

        let panicked = test_results.summary.panics;
        let errored = test_results.summary.failed;
        let passed = test_results.summary.passed;
        let coverage = format!("{:.2}", test_results.summary.coverage);

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
        create_column("Tests ran".into());
        create_column("Passed".green());
        create_column("Failed".red());
        create_column("Panics".red());
        create_column("Coverage".cyan());
        let numbers: Vec<&dyn std::fmt::Display> =
            vec![&num_ran, &passed, &errored, &panicked, &coverage];

        table.print(vec![numbers]);
        if passed > 0 {
            std::process::exit(1);
        } else {
            std::process::exit(0);
        }
    }
=======
pub fn run(query: Option<&str>, pool: Pool, json: bool, show_rast: bool, show_diagnostics: bool) {
	let files = get_test_files(query, &pool, json);
	let num_ran = files.len();

	let detailed = num_ran < 10;

	let pb = indicatif::ProgressBar::new(num_ran as u64);
	pb.set_position(1);
	pb.set_message(&format!("{} tests", "Running".bold().cyan()));
	pb.set_style(default_bar_style());

	std::panic::set_hook(Box::new(|_| {}));
	let start_tests = std::time::Instant::now();
	let mut test_results = TestResults::new();

	let (tx, rx) = std::sync::mpsc::channel();

	pool.scoped(|scope| {
		let pb = &pb;
		for file in files {
			let tx = tx.clone();

			scope.execute(move || {
				let res = run_test_file(file);
				pb.inc(1);

				if detailed && res.fail.is_some() {
					report_detailed_test(pb, &res);
					
				}

				if detailed && show_rast {
					let r = parse_module(&res.code, 0);
					println!("{:#?}", r.syntax());
				}

				if detailed && show_diagnostics {
					let r = parse_module(&res.code, 0);
					let file = rslint_errors::file::SimpleFile::new(
						res.path.display().to_string(), 
						res.code.clone()
					);
					let mut emitter = rslint_errors::Emitter::new(&file);

					for diagnostic in r.errors() {
						emitter.emit_stdout(diagnostic, true).unwrap();
					}
				}

				if detailed && res.fail.is_some() {
					tx.send(res).unwrap();
					return;
				}

				if let Some(ref fail) = res.fail {
					let reason = match fail {
						FailReason::IncorrectlyPassed => "incorrectly passed parsing",
						FailReason::IncorrectlyErrored(_) => "incorrectly threw an error",
						FailReason::ParserPanic(_) => "panicked while parsing",
					};
					let msg = format!(
						"{} '{}' {}",
						"Test".bold().red(),
						res.path
							.strip_prefix("xtask/src/coverage/test262/test/")
							.unwrap_or(&res.path)
							.display(),
						reason.bold()
					);
					pb.println(msg);
				}

				tx.send(res).unwrap();
			});
		}
	});
	drop(tx);

	test_results.store_results(rx.into_iter().collect::<Vec<_>>());

	let _ = std::panic::take_hook();

	pb.finish_and_clear();

	if json {
		test_results.dump_to_json();
	} else {
		println!(
			"\n{} {} tests in {:.2}s\n",
			"Ran".bold().bright_green(),
			num_ran,
			start_tests.elapsed().as_secs_f32()
		);

		let panicked = test_results.summary.panics;
		let errored = test_results.summary.failed;
		let passed = test_results.summary.passed;
		let coverage = format!("{:.2}", test_results.summary.coverage);

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
		create_column("Tests ran".into());
		create_column("Passed".green());
		create_column("Failed".red());
		create_column("Panics".red());
		create_column("Coverage".cyan());
		let numbers: Vec<&dyn std::fmt::Display> =
			vec![&num_ran, &passed, &errored, &panicked, &coverage];

		table.print(vec![numbers]);
		if passed > 0 {
			std::process::exit(1);
		} else {
			std::process::exit(0);
		}
	}
>>>>>>> 36359a9ad (validate unicode escape on string literals)
}

pub fn run_test_file(file: TestFile) -> TestResult {
    let TestFile { code, meta, path } = file;

    if meta.flags.contains(&TestFlag::OnlyStrict) {
        let (code, res) = exec_test(code, true, false);
        let fail = passed(res, meta);
        let outcome = extract_outcome(&fail);
        TestResult {
            fail,
            path,
            code,
            outcome,
        }
    } else if meta.flags.contains(&TestFlag::Module) {
        let (code, res) = exec_test(code, false, true);
        let fail = passed(res, meta);
        let outcome = extract_outcome(&fail);
        TestResult {
            fail,
            path,
            code,
            outcome,
        }
    } else if meta.flags.contains(&TestFlag::NoStrict) || meta.flags.contains(&TestFlag::Raw) {
        let (code, res) = exec_test(code, false, false);
        let fail = passed(res, meta);
        let outcome = extract_outcome(&fail);
        TestResult {
            fail,
            path,
            code,
            outcome,
        }
    } else {
        let (_, l) = exec_test(code.clone(), false, false);
        let (code, r) = exec_test(code, true, false);
        merge_tests(code, l, r, meta, path)
    }
}

fn report_detailed_test(pb: &indicatif::ProgressBar, res: &TestResult) {
	let path = res
		.path
		.strip_prefix("xtask/src/coverage/test262/test/")
		.unwrap_or(&res.path)
		.display();

	let header = format!("\n{} '{}' {}\n", "Test".bold(), path, "failed".bold())
		.red()
		.underline()
		.to_string();

	let msg = match res.fail.as_ref().unwrap() {
		FailReason::IncorrectlyPassed => {
			"    Expected this test to fail, but instead it passed without errors.".into()
		}
		FailReason::ParserPanic(panic) => {
			let msg = panic.as_ref().downcast_ref::<String>();

			let header = format!(
				"    This test caused a{} panic inside the parser{}",
				if msg.is_none() { "n unknown" } else { "" },
				if msg.is_none() { "" } else { ":\n" }
			)
			.bold();

			if let Some(msg) = msg {
				format!(
					"{}    {}\n\n    For more information about the panic run the file manually",
					header, msg
				)
			} else {
				header.to_string()
			}
		}
		FailReason::IncorrectlyErrored(errors) => {
			use rslint_errors::{file::SimpleFile, Emitter};

			let header =
				"    This test threw errors but expected to pass parsing without errors:\n"
					.to_string();
			let file = SimpleFile::new(path.to_string(), res.code.clone());
			let mut emitter = Emitter::new(&file);
			let mut buf = rslint_errors::termcolor::Buffer::ansi();
			for error in errors.iter() {
				emitter
					.emit_with_writer(error, &mut buf)
					.expect("failed to emit error");
			}
			let errors = String::from_utf8(buf.into_inner()).expect("errors are not utf-8");
			format!("{}\n{}", header, errors)
		}
	};
	pb.println(format!("{}{}", header, msg))
}

fn default_bar_style() -> indicatif::ProgressStyle {
    indicatif::ProgressStyle::default_bar()
        .template("{msg} [{bar:40}]")
        .progress_chars("=> ")
}

fn merge_tests(code: String, l: ExecRes, r: ExecRes, meta: MetaData, path: PathBuf) -> TestResult {
    let fail = passed(l, meta.clone()).or_else(|| passed(r, meta));
    // outcome lowers down the reason of failing, being able to use serde over it
    let outcome = extract_outcome(&fail);
    TestResult {
        fail,
        path,
        code,
        outcome,
    }
}

fn passed(res: ExecRes, meta: MetaData) -> Option<FailReason> {
    let should_fail = meta
        .negative
        .filter(|neg| neg.phase == Phase::Parse)
        .is_some();

    match res {
        ExecRes::ParserPanic(msg) => Some(FailReason::ParserPanic(msg)),
        ExecRes::ParseCorrectly if !should_fail => None,
        ExecRes::Errors(_) if should_fail => None,
        ExecRes::ParseCorrectly if should_fail => Some(FailReason::IncorrectlyPassed),
        ExecRes::Errors(err) if !should_fail => Some(FailReason::IncorrectlyErrored(err)),
        _ => unreachable!(),
    }
}

fn extract_outcome(fail: &Option<FailReason>) -> Outcome {
    if let Some(fail) = fail {
        match fail {
            FailReason::IncorrectlyPassed | FailReason::IncorrectlyErrored(_) => Outcome::Failed,
            FailReason::ParserPanic(_) => Outcome::Panicked,
        }
    } else {
        Outcome::Passed
    }
}

enum ExecRes {
    Errors(Vec<ParserError>),
    ParseCorrectly,
    ParserPanic(Box<dyn Any + Send + 'static>),
}

fn exec_test(mut code: String, append_use_strict: bool, module: bool) -> (String, ExecRes) {
    if append_use_strict {
        code.insert_str(0, "\"use strict\";\n");
    }

    let result = std::panic::catch_unwind(|| {
        if module {
            parse_module(&code, 0).ok().map(drop)
        } else {
            parse_text(&code, 0).ok().map(drop)
        }
    });

    let result = result
        .map(|res| {
            if let Err(errors) = res {
                ExecRes::Errors(errors)
            } else {
                ExecRes::ParseCorrectly
            }
        })
        .unwrap_or_else(ExecRes::ParserPanic);

    (code, result)
}
