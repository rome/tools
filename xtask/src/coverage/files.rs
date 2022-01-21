use colored::Colorize;
use indicatif::ProgressBar;
use regex::Regex;
use rslint_parser::ParserError;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fs::read_to_string;
use std::io;
use std::path::PathBuf;
use walkdir::WalkDir;
use yastl::Pool;

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

pub fn get_test_files(query: Option<&str>, pool: &Pool, json: bool) -> Vec<TestFile> {
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
    pb.set_message(format!("{} test files", "Loading".bold().cyan()));
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
                    Some(TestFile { meta, code, path }).filter(|file| {
                        file.meta
                            .negative
                            .as_ref()
                            .map_or(true, |negative| negative.phase == Phase::Parse)
                    })
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
    if !json {
        println!(
            "{} test files in {:.2}s",
            "Loaded".bold().bright_green(),
            start.elapsed().as_secs_f32()
        );
    }

    files
}

