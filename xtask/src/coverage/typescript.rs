use colored::Colorize;
use rslint_parser::parse_text;
use walkdir::{WalkDir, DirEntry};
use yastl::Pool;
use crate::coverage::{ExecRes, TestResult, FailReason, Outcome};
use super::*;

const BASE_PATH: &str = "xtask/src/coverage/Typescript/tests";

pub fn load_ts_files(query: Option<&str>) -> Vec<DirEntry>
{
    WalkDir::new(BASE_PATH)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|file| {
            let path = file.path();
            if !path.is_file() {
                return false;
            }
            let ext = path.extension();
            if ext.is_none() {
                return false;
            }
            if let Some(ext) = ext {
                if ext.to_str() != Some("ts") {
                    return false;
                }
            }
            if let Some(query) = query {
                let path = file.path().to_string_lossy().replace("\\", "/");
                path.contains(query)
            } else {
                true
            }
        })
        .collect::<Vec<_>>()
}

pub fn check_file_encoding(path: &std::path::Path) -> Option<String> {
    let buffer = std::fs::read(path).unwrap();
    let bom = buffer.get(0..3);
    if let Some(&[0xfe, 0xff, _]) = bom {
        //Utf16Be;
        None
    } else if let Some(&[0xff, 0xfe, _]) = bom {
        // Utf16Le
        None
    } else {
        std::str::from_utf8(buffer.as_slice()).ok().map(str::to_string)
    }
}

pub fn run_ts(query: Option<&str>, pool: Pool, json: bool, show_rast: bool, show_diagnostics: bool) {
    let files = load_ts_files(query);

    let pb = indicatif::ProgressBar::new(files.len() as u64);
    let msg = format!("{} tests", "Running".bold().cyan());
    pb.set_position(1);
    pb.set_message(msg);
    pb.set_style(super::default_bar_style());
    
    std::panic::set_hook(Box::new(|_| {}));
    let (tx, rx) = std::sync::mpsc::channel();

    pool.scoped(|scope| {
        let pb = &pb;
        
        for file in files {
            let path = file.path();
            let code = check_file_encoding(path);
            if code.is_none() {
                continue;
            }
            let code = code.unwrap();
            let result = std::panic::catch_unwind(|| {
                parse_text(&code, 0).ok().map(drop)
            });

            let result = result
                .map(|res| {
                    if let Err(errors) = res {
                        TestResult {
                            fail: Some(FailReason::IncorrectlyErrored(errors)),
                            code: code.clone(),
                            outcome: Outcome::Failed,
                            path: path.to_path_buf()
                        }
                    } else {
                        TestResult {
                            fail: None,
                            code: code.clone(),
                            outcome: Outcome::Passed,
                            path: path.to_path_buf()
                        }
                    }
                })
                .unwrap_or_else(|err| {
                    TestResult {
                        fail: Some(FailReason::ParserPanic(err)),
                        code: code.clone(),
                        outcome: Outcome::Panicked,
                        path: path.to_path_buf()
                    }
                });

            tx.send(result);
            pb.inc(1);
        }
    });

    drop(tx);

    let mut test_results = TestResults::new();
    test_results.store_results(rx.into_iter().collect::<Vec<_>>());

    draw_table(&test_results);
    
    if test_results.summary.passed > 0 {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}