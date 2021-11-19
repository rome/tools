use crate::ast::{ArgList, JsRoot};
use crate::{parse_module, parse_text, AstNode, Parse, ParserError, SyntaxNode};
use expect_test::expect_file;
use rslint_errors::file::SimpleFile;
use rslint_errors::termcolor::Buffer;
use rslint_errors::{file::SimpleFiles, Emitter};
use rslint_syntax::SyntaxKind;
use std::fs;
use std::panic::catch_unwind;
use std::path::{Path, PathBuf};

#[test]
fn parser_smoke_test() {
	let src = r#"
    console.log("hello world");
    "#;

	assert!(parse_module(src, 0).ok().is_ok());
}

#[test]
fn parse_file_test() {
	let file_name = r"C:\Users\Micha\git\rome\xtask\src\coverage\test262\test\language\expressions\less-than\S11.8.1_A2.2_T1.js";
	let src = std::fs::read_to_string(Path::new(file_name)).unwrap();

	assert!(parse_module(&src, 0).ok().is_ok());
}

#[test]
fn parser_missing_smoke_test() {
	let src = r#"
		console.log("Hello world";
	"#;

	let module = parse_module(src, 0);

	let arg_list = module
		.syntax()
		.descendants()
		.find_map(ArgList::cast)
		.unwrap();

	let opening = arg_list.syntax.element_in_slot(0);
	let list = arg_list.syntax.element_in_slot(1);
	let closing = arg_list.syntax().element_in_slot(2);

	assert_eq!(opening.map(|o| o.to_string()), Some(String::from("(")));
	assert_eq!(list.map(|l| l.kind()), Some(SyntaxKind::LIST));
	assert_eq!(closing, None);
}

fn test_data_dir() -> PathBuf {
	project_dir().join("rslint_parser/test_data")
}

fn try_parse(path: &str, text: &str) -> Parse<JsRoot> {
	let res = catch_unwind(|| {
		// Files containing a // SCRIPT comment are parsed as script and not as module
		// This is needed to test features that are restricted in strict mode.
		if text.contains("// SCRIPT") {
			parse_text(text, 0)
		} else {
			parse_module(text, 0)
		}
	});
	assert!(
		!res.is_err(),
		"Trying to parse `{}` caused infinite recursion",
		path
	);
	res.unwrap()
}

#[test]
fn parser_tests() {
	dir_tests(&test_data_dir(), &["inline/ok"], "rast", |text, path| {
		let parse = try_parse(path.to_str().unwrap(), text);
		let errors = parse.errors();
		assert_errors_are_absent(errors, path, &parse.syntax());
		format!("{:#?}", parse.syntax())
	});

	dir_tests(&test_data_dir(), &["inline/err"], "rast", |text, path| {
		let parse = try_parse(path.to_str().unwrap(), text);
		let errors = parse.errors();
		assert_errors_are_present(errors, path, &parse.syntax());
		let mut files = SimpleFiles::new();
		files.add(
			path.file_name().unwrap().to_string_lossy().to_string(),
			text.to_string(),
		);
		let mut ret = format!("{:#?}", parse.syntax());

		for diag in parse.errors() {
			let mut write = rslint_errors::termcolor::Buffer::no_color();
			let mut emitter = Emitter::new(&files);
			emitter
				.emit_with_writer(diag, &mut write)
				.expect("failed to emit diagnostic");

			ret.push_str(&format!(
				"--\n{}",
				std::str::from_utf8(write.as_slice()).expect("non utf8 in error buffer")
			));
		}
		ret.push_str(&format!("--\n{}", text));
		ret
	});
}

fn dir_tests<F>(test_data_dir: &Path, paths: &[&str], outfile_extension: &str, f: F)
where
	F: Fn(&str, &Path) -> String,
{
	for (path, input_code) in collect_js_files(test_data_dir, paths) {
		let actual = f(&input_code, &path);
		let path = path.with_extension(outfile_extension);
		expect_file![path].assert_eq(&actual)
	}
}

pub fn project_dir() -> PathBuf {
	let dir = env!("CARGO_MANIFEST_DIR");
	PathBuf::from(dir).parent().unwrap().to_path_buf()
}

fn collect_js_files(root_dir: &Path, paths: &[&str]) -> Vec<(PathBuf, String)> {
	paths
		.iter()
		.flat_map(|path| {
			let path = root_dir.to_owned().join(path);
			js_files_in_dir(&path).into_iter()
		})
		.map(|path| {
			let text = fs::read_to_string(&path).expect("Could not read js file");
			(path, text)
		})
		.collect()
}

fn js_files_in_dir(dir: &Path) -> Vec<PathBuf> {
	let mut acc = Vec::new();
	println!("{:?}", dir);
	for file in fs::read_dir(&dir).unwrap() {
		let file = file.unwrap();
		let path = file.path();
		if path.extension().unwrap_or_default() == "js" {
			acc.push(path);
		}
	}
	acc.sort();
	acc
}

fn assert_errors_are_present(errors: &[ParserError], path: &Path, syntax: &SyntaxNode) {
	assert!(
		!errors.is_empty(),
		"There should be errors in the file {:?}\nSyntax Tree: {:#?}",
		path.display(),
		syntax
	);
}

fn assert_errors_are_absent(errors: &[ParserError], path: &Path, syntax: &SyntaxNode) {
	if errors.is_empty() {
		return;
	}

	let file = SimpleFile::new(path.to_str().unwrap().to_string(), syntax.to_string());
	let mut emitter = Emitter::new(&file);
	let mut buffer = Buffer::no_color();

	for diagnostic in errors {
		emitter.emit_with_writer(diagnostic, &mut buffer).unwrap();
	}

	panic!("There should be no errors in the file {:?} but the following errors where present:\n{}\n\nParsed tree:\n{:#?}",
		path.display(),
		std::str::from_utf8(buffer.as_slice()).unwrap(),
		syntax
	);
}
