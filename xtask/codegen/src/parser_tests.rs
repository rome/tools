//! Takes comments from rslint_parser and turns them into test data.
//! This code is derived from rust_analyzer/xtask/codegen/gen_parser_tests

use std::{
    collections::HashMap,
    fs, iter, mem,
    path::{Path, PathBuf},
};

use crate::{update, Mode};
use xtask::{project_root, Result};

fn extract_comment_blocks(
    text: &str,
    allow_blocks_with_empty_lines: bool,
) -> Vec<(usize, Vec<String>)> {
    let mut res = Vec::new();

    let prefix = "// ";
    let lines = text.lines().map(str::trim_start);

    let mut block = (0, vec![]);
    for (line_num, line) in lines.enumerate() {
        if line == "//" && allow_blocks_with_empty_lines {
            block.1.push(String::new());
            continue;
        }

        let is_comment = line.starts_with(prefix);
        if is_comment {
            block.1.push(line[prefix.len()..].to_string());
        } else {
            if !block.1.is_empty() {
                res.push(mem::take(&mut block));
            }
            block.0 = line_num + 2;
        }
    }
    if !block.1.is_empty() {
        res.push(block)
    }
    res
}

pub fn generate_parser_tests(mode: Mode) -> Result<()> {
    let tests = tests_from_dir(&project_root().join(Path::new("crates/rslint_parser/src/syntax")))?;
    fn install_tests(tests: &HashMap<String, Test>, into: &str, mode: Mode) -> Result<()> {
        let tests_dir = project_root().join(into);
        if !tests_dir.is_dir() {
            fs::create_dir_all(&tests_dir)?;
        }
        // ok is never actually read, but it needs to be specified to create a Test in existing_tests
        let existing = existing_tests(&tests_dir, true)?;
        for t in existing.keys().filter(|&t| !tests.contains_key(t)) {
            panic!("Test is deleted: {}", t);
        }

        for (name, test) in tests {
            let path = match existing.get(name) {
                Some((path, _test)) => path.clone(),
                None => {
                    let file_name = format!("{}.{}", name, test.language);
                    tests_dir.join(file_name)
                }
            };
            update(&path, &test.text, mode)?;
        }
        Ok(())
    }
    install_tests(&tests.ok, "crates/rslint_parser/test_data/inline/ok", mode)?;
    install_tests(
        &tests.err,
        "crates/rslint_parser/test_data/inline/err",
        mode,
    )
}

#[derive(Debug)]
struct Test {
    pub name: String,
    pub language: String,
    pub text: String,
    pub ok: bool,
}

#[derive(Default, Debug)]
struct Tests {
    pub ok: HashMap<String, Test>,
    pub err: HashMap<String, Test>,
}

fn collect_tests(s: &str) -> Vec<Test> {
    let mut res = Vec::new();
    for comment_block in extract_comment_blocks(s, false).into_iter().map(|(_, x)| x) {
        let first_line = &comment_block[0];
        let (language, name, ok) = if let Some(first_line) = first_line.strip_prefix("test ts ") {
            let name = first_line.to_string();
            ("ts", name, true)
        } else if let Some(first_line) = first_line.strip_prefix("test_err ts ") {
            let name = first_line.to_string();
            ("ts", name, false)
        } else if let Some(first_line) = first_line.strip_prefix("test ") {
            let name = first_line.to_string();
            ("js", name, true)
        } else if let Some(first_line) = first_line.strip_prefix("test_err ") {
            let name = first_line.to_string();
            ("js", name, false)
        } else {
            continue;
        };
        let text: String = comment_block[1..]
            .iter()
            .cloned()
            .chain(iter::once(String::new()))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(!text.trim().is_empty() && text.ends_with('\n'));
        res.push(Test {
            name,
            text,
            ok,
            language: language.to_string(),
        })
    }
    res
}

fn tests_from_dir(dir: &Path) -> Result<Tests> {
    let mut res = Tests::default();
    for entry in ::walkdir::WalkDir::new(dir) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().unwrap_or_default() != "rs" {
            continue;
        }
        process_file(&mut res, entry.path())?;
    }
    return Ok(res);
    fn process_file(res: &mut Tests, path: &Path) -> Result<()> {
        let text = fs::read_to_string(path)?;

        for test in collect_tests(&text) {
            if test.ok {
                if let Some(old_test) = res.ok.insert(test.name.clone(), test) {
                    anyhow::bail!("Duplicate test: {}", old_test.name);
                }
            } else if let Some(old_test) = res.err.insert(test.name.clone(), test) {
                anyhow::bail!("Duplicate test: {}", old_test.name);
            }
        }
        Ok(())
    }
}

fn existing_tests(dir: &Path, ok: bool) -> Result<HashMap<String, (PathBuf, Test)>> {
    let mut res = HashMap::new();
    for file in fs::read_dir(dir)? {
        let file = file?;
        let path = file.path();
        let exts = ["js", "ts"];
        let ext = path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        if !exts.contains(&ext) {
            continue;
        }
        let name = {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            file_name[..file_name.len() - 3].to_string()
        };
        let text = fs::read_to_string(&path)?;
        let test = Test {
            name: name.clone(),
            text,
            ok,
            language: ext.to_string(),
        };
        if let Some(old) = res.insert(name, (path, test)) {
            println!("Duplicate test: {:?}", old);
        }
    }
    Ok(res)
}
