use crate::runner::{TestCase, TestRunOutcome, TestSuite};
use regex::Regex;
use rslint_parser::{parse, Syntax};
use serde::Deserialize;
use std::io;
use std::path::{Path, PathBuf};

const BASE_PATH: &str = "xtask/coverage/test262/test";

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

#[derive(Debug)]
struct Test262TestCase {
    path: PathBuf,
    code: String,
    meta: MetaData,
}

impl Test262TestCase {
    fn execute_test(&self, append_use_strict: bool, syntax: Syntax) -> TestRunOutcome {
        let parse_result = if append_use_strict {
            let code = format!("\"use strict\";\n{}", self.code);
            parse(&code, 0, syntax).ok()
        } else {
            parse(&self.code, 0, syntax).ok()
        };

        let should_fail = self
            .meta
            .negative
            .as_ref()
            .filter(|neg| neg.phase == Phase::Parse)
            .is_some();

        match parse_result {
            Ok(_) if !should_fail => TestRunOutcome::Passed(syntax),
            Err(_) if should_fail => TestRunOutcome::Passed(syntax),
            Ok(_) if should_fail => TestRunOutcome::IncorrectlyPassed(syntax),
            Err(errors) if !should_fail => TestRunOutcome::IncorrectlyErrored { errors, syntax },
            _ => unreachable!(),
        }
    }
}

impl TestCase for Test262TestCase {
    fn path(&self) -> &Path {
        self.path.strip_prefix(BASE_PATH).unwrap()
    }

    fn code(&self) -> &str {
        &self.code
    }

    fn run(&self) -> TestRunOutcome {
        let meta = &self.meta;
        if meta.flags.contains(&TestFlag::OnlyStrict) {
            self.execute_test(true, Syntax::default())
        } else if meta.flags.contains(&TestFlag::Module) {
            self.execute_test(false, Syntax::default().module())
        } else if meta.flags.contains(&TestFlag::NoStrict) || meta.flags.contains(&TestFlag::Raw) {
            self.execute_test(false, Syntax::default())
        } else {
            let l = self.execute_test(false, Syntax::default());
            let r = self.execute_test(true, Syntax::default());
            merge_outcomes(l, r)
        }
    }
}

pub(crate) struct Test262TestSuite;

impl TestSuite for Test262TestSuite {
    fn name(&self) -> &str {
        "T262"
    }

    fn base_path(&self) -> &str {
        BASE_PATH
    }

    fn is_test(&self, path: &Path) -> bool {
        match path.extension() {
            None => false,
            Some(ext) => ext == "js",
        }
    }

    fn load_test(&self, path: PathBuf) -> Option<Box<dyn TestCase>> {
        let code = std::fs::read_to_string(&path).ok()?;

        let meta = read_metadata(&code).ok()?;

        if !meta
            .negative
            .as_ref()
            .map_or(true, |negative| negative.phase == Phase::Parse)
        {
            None
        } else {
            Some(Box::new(Test262TestCase { code, path, meta }))
        }
    }
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

// use super::{files::*, *};
// use colored::Colorize;
// use rslint_parser::{parse, parse_module, Syntax};
// use std::path::PathBuf;
// use yastl::Pool;
//
// pub const TEST_JSON_PATH: &str = "xtask/src/base_results.json";
//
// pub fn run_js(
//     query: Option<&str>,
//     pool: Pool,
//     json: bool,
//     show_rast: bool,
//     show_diagnostics: bool,
// ) {
//     let files = get_test_files(query, &pool, json);
//     let num_ran = files.len();
//
//     let detailed = num_ran < 10;
//
//     let pb = indicatif::ProgressBar::new(num_ran as u64);
//     let msg = format!("{} tests", "Running".bold().cyan());
//     pb.set_position(1);
//     pb.set_message(msg);
//     pb.set_style(super::default_bar_style());
//
//     std::panic::set_hook(Box::new(|_| {}));
//     let start_tests = std::time::Instant::now();
//     let mut test_results = TestResults::new();
//
//     let (tx, rx) = std::sync::mpsc::channel();
//
//     pool.scoped(|scope| {
//         let pb = &pb;
//         for file in files {
//             let tx = tx.clone();
//
//             scope.execute(move || {
//                 let res = run_test_file(file);
//                 pb.inc(1);
//
//                 if detailed && res.fail.is_some() {
//                     report_detailed_test(pb, &res);
//                 }
//
//                 if detailed && show_rast {
//                     let r = parse_module(&res.code, 0);
//                     println!("{:#?}", r.syntax());
//                 }
//
//                 if detailed && show_diagnostics {
//                     let r = parse_module(&res.code, 0);
//                     let file = rslint_errors::file::SimpleFile::new(
//                         res.path.display().to_string(),
//                         res.code.clone(),
//                     );
//                     let mut emitter = rslint_errors::Emitter::new(&file);
//
//                     for diagnostic in r.errors() {
//                         emitter.emit_stdout(diagnostic, true).unwrap();
//                     }
//                 }
//
//                 if detailed && res.fail.is_some() {
//                     tx.send(res).unwrap();
//                     return;
//                 }
//
//                 if let Some(ref fail) = res.fail {
//                     let reason = match fail {
//                         FailReason::IncorrectlyPassed => "incorrectly passed parsing",
//                         FailReason::IncorrectlyErrored(_) => "incorrectly threw an error",
//                         FailReason::ParserPanic(_) => "panicked while parsing",
//                     };
//                     let msg = format!(
//                         "{} '{}' {}",
//                         "Test".bold().red(),
//                         res.path
//                             .strip_prefix("xtask/src/coverage/test262/test/")
//                             .unwrap_or(&res.path)
//                             .display(),
//                         reason.bold()
//                     );
//                     pb.println(msg);
//                 }
//
//                 tx.send(res).unwrap();
//             });
//         }
//     });
//     drop(tx);
//
//     test_results.store_results(rx.into_iter().collect::<Vec<_>>());
//
//     let _ = std::panic::take_hook();
//
//     pb.finish_and_clear();
//
//     if json {
//         test_results.dump_to_json();
//     } else {
//         println!(
//             "\n{} {} tests in {:.2}s\n",
//             "Ran".bold().bright_green(),
//             num_ran,
//             start_tests.elapsed().as_secs_f32()
//         );
//
//         super::draw_table(&test_results);
//
//         if test_results.summary.passed > 0 {
//             std::process::exit(1);
//         } else {
//             std::process::exit(0);
//         }
//     }
// }
//
// pub fn run_test_file(file: TestFile) -> TestSummary {
//     let TestFile { code, meta, path } = file;
//
//     if meta.flags.contains(&TestFlag::OnlyStrict) {
//         let (code, res) = exec_test(code, true, Syntax::default());
//         let fail = passed(res, meta);
//         let outcome = extract_outcome(&fail);
//         TestSummary {
//             fail,
//             path,
//             code,
//             outcome,
//         }
//     } else if meta.flags.contains(&TestFlag::Module) {
//         let (code, res) = exec_test(code, false, Syntax::default().module());
//         let fail = passed(res, meta);
//         let outcome = extract_outcome(&fail);
//         TestSummary {
//             fail,
//             path,
//             code,
//             outcome,
//         }
//     } else if meta.flags.contains(&TestFlag::NoStrict) || meta.flags.contains(&TestFlag::Raw) {
//         let (code, res) = exec_test(code, false, Syntax::default());
//         let fail = passed(res, meta);
//         let outcome = extract_outcome(&fail);
//         TestSummary {
//             fail,
//             path,
//             code,
//             outcome,
//         }
//     } else {
//         let (_, l) = exec_test(code.clone(), false, Syntax::default());
//         let (code, r) = exec_test(code, true, Syntax::default());
//         merge_tests(code, l, r, meta, path)
//     }
// }
//
// fn report_detailed_test(pb: &indicatif::ProgressBar, res: &TestSummary) {
//     let path = res
//         .path
//         .strip_prefix("xtask/src/coverage/test262/test/")
//         .unwrap_or(&res.path)
//         .display();
//
//     let header = format!("\n{} '{}' {}\n", "Test".bold(), path, "failed".bold())
//         .red()
//         .underline()
//         .to_string();
//
//     let msg = match res.fail.as_ref().unwrap() {
//         FailReason::IncorrectlyPassed => {
//             "    Expected this test to fail, but instead it passed without errors.".into()
//         }
//         FailReason::ParserPanic(panic) => {
//             let msg = panic.as_ref().downcast_ref::<String>();
//
//             let header = format!(
//                 "    This test caused a{} panic inside the parser{}",
//                 if msg.is_none() { "n unknown" } else { "" },
//                 if msg.is_none() { "" } else { ":\n" }
//             )
//             .bold();
//
//             if let Some(msg) = msg {
//                 format!(
//                     "{}    {}\n\n    For more information about the panic run the file manually",
//                     header, msg
//                 )
//             } else {
//                 header.to_string()
//             }
//         }
//         FailReason::IncorrectlyErrored(errors) => {
//             use rslint_errors::{file::SimpleFile, Emitter};
//
//             let header =
//                 "    This test threw errors but expected to pass parsing without errors:\n"
//                     .to_string();
//             let file = SimpleFile::new(path.to_string(), res.code.clone());
//             let mut emitter = Emitter::new(&file);
//             let mut buf = rslint_errors::termcolor::Buffer::ansi();
//             for error in errors.iter() {
//                 emitter
//                     .emit_with_writer(error, &mut buf)
//                     .expect("failed to emit error");
//             }
//             let errors = String::from_utf8(buf.into_inner()).expect("errors are not utf-8");
//             format!("{}\n{}", header, errors)
//         }
//     };
//     pb.println(format!("{}{}", header, msg))
// }
//
fn merge_outcomes(l: TestRunOutcome, r: TestRunOutcome) -> TestRunOutcome {
    let result = if l.is_failed() { l } else { r };

    result
}
