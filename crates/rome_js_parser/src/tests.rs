use crate::{parse, parse_module, Parse, SourceType};
use expect_test::expect_file;
use rome_diagnostics::file::SimpleFile;
use rome_diagnostics::termcolor::Buffer;
use rome_diagnostics::{file::SimpleFiles, Emitter};
use rome_js_syntax::{
    AstNode, JsCallArguments, JsLogicalExpression, SyntaxNode, SyntaxNodeExt, SyntaxToken,
};
use rome_js_syntax::{JsAnyRoot, JsSyntaxKind};
use rome_rowan::{SyntaxKind, TextSize};
use std::panic::catch_unwind;
use std::path::{Path, PathBuf};

#[test]
fn parser_smoke_test() {
    let src = r#"
let a = <Test />;
"#;

    let module = parse(src, 0, SourceType::tsx());
    assert_errors_are_absent(&module, Path::new("parser_smoke_test"));
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

    let opening = arg_list.syntax().element_in_slot(0);
    let list = arg_list.syntax().element_in_slot(1);
    let closing = arg_list.syntax().element_in_slot(2);

    assert_eq!(opening.map(|o| o.to_string()), Some(String::from("(")));
    assert_eq!(
        list.map(|l| l.kind()),
        Some(JsSyntaxKind::JS_CALL_ARGUMENT_LIST)
    );
    assert_eq!(closing, None);
}

fn try_parse(path: &str, text: &str) -> Parse<JsAnyRoot> {
    let res = catch_unwind(|| {
        let path = Path::new(path);
        // Files containing a // SCRIPT comment are parsed as script and not as module
        // This is needed to test features that are restricted in strict mode.
        let source_type = if text.contains("// SCRIPT") {
            SourceType::js_script()
        } else {
            path.try_into().unwrap()
        };

        let parse = parse(text, 0, source_type);

        assert_eq!(
            parse.syntax().to_string(),
            text,
            "Original source and re-printed tree differ\nParsed Tree: {:#?}",
            parse.syntax(),
        );

        parse
    });
    assert!(res.is_ok(), "Trying to parse `{}` panicked", path);
    res.unwrap()
}

fn try_parse_with_printed_ast(path: &str, text: &str) -> (Parse<JsAnyRoot>, String) {
    catch_unwind(|| {
        let parse = try_parse(path, text);
        let formatted = format!("{:#?}", &parse.tree());
        (parse, formatted)
    })
    .unwrap_or_else(|err| {
        // Re-parsing the source here seems silly. But the problem is, that `SyntaxNode`s aren't
        // unwind safe. That's why the same `ParseResult` can't be reused here.
        // This should be fine because this code is only executed for local tests. No checked-in
        // test should ever hit this line.
        let re_parsed = try_parse(path, text);
        panic!(
            "Printing the AST for `{}` panicked. That means it is malformed. Err: {:?}\n{:#?}",
            path,
            err,
            re_parsed.syntax()
        );
    })
}

#[cfg(test)]
fn run_and_expect_no_errors(path: &str, _: &str, _: &str, _: &str) {
    let path = PathBuf::from(path);
    let text = std::fs::read_to_string(&path).unwrap();

    let (parse, ast) = try_parse_with_printed_ast(path.to_str().unwrap(), &text);
    assert_errors_are_absent(&parse, &path);
    let actual = format!("{}\n\n{:#?}", ast, parse.syntax());

    let path = path.with_extension("rast");
    expect_file![path].assert_eq(&actual)
}

#[cfg(test)]
fn run_and_expect_errors(path: &str, _: &str, _: &str, _: &str) {
    let path = PathBuf::from(path);
    let text = std::fs::read_to_string(&path).unwrap();

    let (parse, ast) = try_parse_with_printed_ast(path.to_str().unwrap(), &text);
    assert_errors_are_present(&parse, &path);
    let mut files = SimpleFiles::new();
    files.add(
        path.file_name().unwrap().to_string_lossy().to_string(),
        text.to_string(),
    );
    let mut actual = format!("{}\n\n{:#?}", ast, parse.syntax());
    for diag in parse.diagnostics() {
        let mut write = rome_diagnostics::termcolor::Buffer::no_color();
        let mut emitter = Emitter::new(&files);
        emitter
            .emit_with_writer(diag, &mut write)
            .expect("failed to emit diagnostic");
        actual.push_str(&format!(
            "--\n{}",
            std::str::from_utf8(write.as_slice()).expect("non utf8 in error buffer")
        ));
    }
    actual.push_str(&format!("--\n{}", text));

    let path = path.with_extension("rast");
    expect_file![path].assert_eq(&actual)
}

mod parser {
    mod ok {
        tests_macros::gen_tests! {"test_data/inline/ok/**/*.{js,ts,jsx,tsx}", crate::tests::run_and_expect_no_errors, ""}
    }
    mod err {
        tests_macros::gen_tests! {"test_data/inline/err/**/*.{js,ts,jsx,tsx}", crate::tests::run_and_expect_errors, ""}
    }
}

fn assert_errors_are_present(program: &Parse<JsAnyRoot>, path: &Path) {
    assert!(
        !program.diagnostics().is_empty(),
        "There should be errors in the file {:?}\nSyntax Tree: {:#?}",
        path.display(),
        program.syntax()
    );
}

// sometimes our parser emits unknown nodes without diagnostics;
// this check makes sure that we don't signal that the tree has errors.
fn has_unknown_nodes(node: &SyntaxNode) -> bool {
    node.descendants()
        .any(|descendant| descendant.kind().is_unknown())
}

fn assert_errors_are_absent<T>(program: &Parse<T>, path: &Path) {
    let syntax = program.syntax();
    if !program.has_errors() && !has_unknown_nodes(&syntax) {
        return;
    }

    let file = SimpleFile::new(path.to_str().unwrap().to_string(), syntax.to_string());
    let mut emitter = Emitter::new(&file);
    let mut buffer = Buffer::no_color();

    for diagnostic in program.diagnostics() {
        emitter.emit_with_writer(diagnostic, &mut buffer).unwrap();
    }

    panic!("There should be no errors in the file {:?} but the following errors where present:\n{}\n\nParsed tree:\n{:#?}",
        path.display(),
        std::str::from_utf8(buffer.as_slice()).unwrap(),
        &syntax
	);
}

#[test]
pub fn test_trivia_attached_to_tokens() {
    let text = "/**/let a = 1; // nice variable \n /*hey*/ let \t b = 2; // another nice variable";
    let m = parse_module(text, 0);
    let mut tokens = m.syntax().descendants_tokens();

    let is_let = |x: &SyntaxToken| x.text_trimmed() == "let";
    let first_let = tokens.find(is_let).unwrap();

    // first let leading trivia asserts
    let pieces: Vec<_> = first_let.leading_trivia().pieces().collect();
    assert!(matches!(pieces.get(0).map(|x| x.text()), Some("/**/")));
    assert!(matches!(pieces.get(1), None));

    // first let trailing trivia asserts
    let pieces: Vec<_> = first_let.trailing_trivia().pieces().collect();
    assert!(matches!(pieces.get(0).map(|x| x.text()), Some(" ")));
    assert!(matches!(pieces.get(1), None));

    // second let leading trivia asserts
    let second_let = tokens.find(is_let).unwrap();
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

#[test]
pub fn node_contains_comments() {
    let text = "true && true // comment";
    let root = parse_module(text, 0);
    let syntax = root.syntax();

    assert!(syntax.contains_comments());
}

#[test]
fn parser_regexp_after_operator() {
    fn assert_no_errors(src: &str) {
        let module = parse(src, 0, SourceType::js_script());
        assert_errors_are_absent(&module, Path::new("parser_regexp_after_operator"));
    }
    assert_no_errors(r#"a=/a/"#);
    assert_no_errors(r#"a==/a/"#);
    assert_no_errors(r#"a===/a/"#);
    assert_no_errors(r#"a!=/a/"#);
    assert_no_errors(r#"a!==/a/"#);
}

#[test]
pub fn node_contains_trailing_comments() {
    let text = "true && (3 - 2 == 0) // comment";
    let root = parse_module(text, 0);
    let syntax = root.syntax();
    let node = syntax
        .descendants()
        .find(|n| n.kind() == JsSyntaxKind::JS_LOGICAL_EXPRESSION)
        .unwrap();

    let logical_expression = JsLogicalExpression::cast(node).unwrap();
    let right = logical_expression.right().unwrap();

    assert!(right.syntax().has_trailing_comments());
    assert!(!right.syntax().has_leading_comments());
}

#[test]
pub fn node_contains_leading_comments() {
    let text = r"true &&
// comment
(3 - 2 == 0)";
    let root = parse_module(text, 0);
    let syntax = root.syntax();
    let node = syntax
        .descendants()
        .find(|n| n.kind() == JsSyntaxKind::JS_LOGICAL_EXPRESSION)
        .unwrap();

    let logical_expression = JsLogicalExpression::cast(node).unwrap();
    let right = logical_expression.right().unwrap();

    assert!(right.syntax().has_leading_comments());
    assert!(!right.syntax().has_trailing_comments());
}
