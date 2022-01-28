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

fn merge_outcomes(l: TestRunOutcome, r: TestRunOutcome) -> TestRunOutcome {
    if l.is_failed() {
        l
    } else {
        r
    }
}
