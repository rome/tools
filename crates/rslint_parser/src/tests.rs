use crate::ast::{JsAnyRoot, JsCallArguments};
use crate::{parse_module, parse_text, AstNode, Parse, ParserError, SyntaxNode, SyntaxToken};
use expect_test::expect_file;
use rome_rowan::TextSize;
use rslint_errors::file::SimpleFile;
use rslint_errors::termcolor::Buffer;
use rslint_errors::{file::SimpleFiles, Emitter};
use rslint_syntax::JsSyntaxKind;
use std::fs;
use std::panic::catch_unwind;
use std::path::{Path, PathBuf};

#[test]
fn parser_smoke_test() {
	let src = r#"
let [a, b] = [1, 2];
    "#;

	let module = parse_module(src, 0);

	assert_errors_are_absent(
		module.errors(),
		Path::new("parser_smoke_test"),
		&module.syntax(),
	);
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
		.find_map(JsCallArguments::cast)
		.unwrap();

	let opening = arg_list.syntax.element_in_slot(0);
	let list = arg_list.syntax.element_in_slot(1);
	let closing = arg_list.syntax().element_in_slot(2);

	assert_eq!(opening.map(|o| o.to_string()), Some(String::from("(")));
	assert_eq!(
		list.map(|l| l.kind()),
		Some(JsSyntaxKind::JS_CALL_ARGUMENT_LIST)
	);
	assert_eq!(closing, None);
}

fn test_data_dir() -> PathBuf {
	project_dir().join("rslint_parser/test_data")
}

#[test]
fn test_eval() {
	println!("{:?}", try_parse("", " ({ eval }) = 0"));
}

fn try_parse(path: &str, text: &str) -> Parse<JsAnyRoot> {
	let res = catch_unwind(|| {
		// Files containing a // SCRIPT comment are parsed as script and not as module
		// This is needed to test features that are restricted in strict mode.
		let parse = if text.contains("// SCRIPT") {
			parse_text(text, 0).cast::<JsAnyRoot>().unwrap()
		} else {
			parse_module(text, 0).cast::<JsAnyRoot>().unwrap()
		};

		assert_eq!(
			parse.syntax().to_string(),
			text,
			"Original source and re-printed tree differ\nParsed Tree: {:#?}",
			parse.syntax(),
		);

		parse
	});
	assert!(!res.is_err(), "Trying to parse `{}` panicked", path);
	res.unwrap()
}

fn try_parse_with_printed_ast(path: &str, text: &str) -> (Parse<JsAnyRoot>, String) {
	catch_unwind(|| {
		let parse = try_parse(path, text);
		let formatted = format!("{:#?}", &parse.tree());
		(parse, formatted)
	})
	.unwrap_or_else(|_| {
		// Re-parsing the source here seems silly. But the problem is, that `SyntaxNode`s aren't
		// unwind safe. That's why the same `ParseResult` can't be reused here.
		// This should be fine because this code is only executed for local tests. No checked-in
		// test should ever hit this line.
		let re_parsed = try_parse(path, text);
		panic!(
			"Printing the AST for `{}` panicked. That means it is malformed.\n{:#?}",
			path,
			re_parsed.syntax()
		);
	})
}

#[test]
fn parser_tests() {
	dir_tests(&test_data_dir(), &["inline/ok"], "rast", |text, path| {
		let (parse, ast) = try_parse_with_printed_ast(path.to_str().unwrap(), text);
		let errors = parse.errors();
		assert_errors_are_absent(errors, path, &parse.syntax());

		format!("{}\n\n{:#?}", ast, parse.syntax())
	});

	dir_tests(&test_data_dir(), &["inline/err"], "rast", |text, path| {
		let (parse, ast) = try_parse_with_printed_ast(path.to_str().unwrap(), text);
		let errors = parse.errors();
		assert_errors_are_present(errors, path, &parse.syntax());
		let mut files = SimpleFiles::new();
		files.add(
			path.file_name().unwrap().to_string_lossy().to_string(),
			text.to_string(),
		);
		let mut ret = format!("{}\n\n{:#?}", ast, parse.syntax());

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

#[test]
pub fn test_trivia_attached_to_tokens() {
	use crate::util::SyntaxNodeExt;

	let text = "/**/let a = 1; // nice variable \n /*hey*/ let \t b = 2; // another nice variable";
	let m = parse_module(text, 0);
	let tokens = m.syntax().tokens();

	let is_let = |x: &&SyntaxToken| x.text_trimmed() == "let";
	let first_let = tokens.iter().find(is_let).unwrap();

	// first let leading trivia asserts
	let pieces: Vec<_> = first_let.leading_trivia().pieces().collect();
	assert!(matches!(pieces.get(0).map(|x| x.text()), Some("/**/")));
	assert!(matches!(pieces.get(1), None));

	// first let trailing trivia asserts
	let pieces: Vec<_> = first_let.trailing_trivia().pieces().collect();
	assert!(matches!(pieces.get(0).map(|x| x.text()), Some(" ")));
	assert!(matches!(pieces.get(1), None));

	// second let leading trivia asserts
	let second_let = tokens.iter().filter(is_let).nth(1).unwrap();
	let pieces: Vec<_> = second_let.leading_trivia().pieces().collect();
	assert_eq!(4, pieces.len());
	assert!(matches!(pieces.get(0).map(|x| x.text()), Some("\n")));
	assert!(matches!(pieces.get(1).map(|x| x.text()), Some(" ")));
	assert!(matches!(pieces.get(2).map(|x| x.text()), Some("/*hey*/")));
	assert!(matches!(pieces.get(3).map(|x| x.text()), Some(" ")));

	// second let trailing trivia asserts
	let pieces: Vec<_> = second_let.trailing_trivia().pieces().collect();
	assert_eq!(1, pieces.len());
	assert!(matches!(pieces.get(0).map(|x| x.text()), Some(" \t ")));
}

#[test]
pub fn jsroot_display_text_and_trimmed() {
	let code = " let a = 1; \n ";
	let root = parse_module(code, 0);
	let syntax = root.syntax();

	assert_eq!(format!("{}", syntax), code);

	let syntax_text = syntax.text();
	assert_eq!(format!("{}", syntax_text), code);

	let syntax_text = syntax.text_trimmed();
	assert_eq!(format!("{}", syntax_text), code.trim());
}

#[test]
pub fn jsroot_ranges() {
	//               0123456789A
	let code = " let a = 1;";
	let root = parse_module(code, 0);
	let syntax = root.syntax();

	let first_let = syntax.first_token().unwrap();
	let range = first_let.text_range();
	assert_eq!(0usize, range.start().into());
	assert_eq!(5usize, range.end().into());

	let range = first_let.text_trimmed_range();
	assert_eq!(1usize, range.start().into());
	assert_eq!(4usize, range.end().into());

	let eq = syntax
		.descendants_tokens()
		.find(|x| x.text_trimmed() == "=")
		.unwrap();
	let range = eq.text_range();
	assert_eq!(7usize, range.start().into());
	assert_eq!(9usize, range.end().into());

	let range = eq.text_trimmed_range();
	assert_eq!(7usize, range.start().into());
	assert_eq!(8usize, range.end().into());
}

#[test]
pub fn node_range_must_be_correct() {
	//               0123456789A123456789B123456789
	let text = " function foo() { let a = 1; }";
	let root = parse_module(text, 0);

	let var_decl = root
		.syntax()
		.descendants()
		.find(|x| x.kind() == JsSyntaxKind::JS_VARIABLE_STATEMENT)
		.unwrap();

	let range = var_decl.text_range();
	assert_eq!(18usize, range.start().into());
	assert_eq!(29usize, range.end().into());

	let range = var_decl.text_trimmed_range();
	assert_eq!(18usize, range.start().into());
	assert_eq!(28usize, range.end().into());
}

#[test]
pub fn last_trivia_must_be_appended_to_eof() {
	//               0123456789A123456789B123456789CC
	let text = " function foo() { let a = 1; }\n";
	let root = parse_module(text, 0);
	let syntax = root.syntax();

	let range = syntax.text_range();
	let start = range.start();
	let end = range.end();

	assert_eq!(TextSize::from(0), start);
	assert_eq!(TextSize::from(31), end);
}

#[test]
pub fn just_trivia_must_be_appended_to_eof() {
	//               0123456789A123456789B123456789C123
	let text = "// just trivia... nothing else....";
	let root = parse_module(text, 0);
	let syntax = root.syntax();

	let range = syntax.text_range();
	let start = range.start();
	let end = range.end();

	assert_eq!(TextSize::from(0), start);
	assert_eq!(TextSize::from(34), end);
}
