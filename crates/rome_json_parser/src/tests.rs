use expect_test::expect_file;
use rome_diagnostics::file::SimpleFile;
use rome_diagnostics::termcolor::Buffer;
use rome_diagnostics::{file::SimpleFiles, Emitter};
use rome_json_syntax::{JsonLanguage, JsonRoot, JsonSyntaxNode};
use rome_rowan::{AstNode, Direction, SyntaxKind, TextSize};
use std::fmt::Debug;
use std::panic::catch_unwind;
use std::path::{Path, PathBuf};

use crate::parse::{self, Parse};
fn try_parse(path: &str, text: &str) -> Parse<JsonRoot> {
    let res = catch_unwind(|| {
        let parse = parse::parse(text, 0);

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

fn try_parse_with_printed_ast(path: &str, text: &str) -> (Parse<JsonRoot>, String) {
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
        tests_macros::gen_tests! {"test_data/inline/ok/**/*.{json}", crate::tests::run_and_expect_no_errors, ""}
    }
    mod err {
        tests_macros::gen_tests! {"test_data/inline/err/**/*.{json}", crate::tests::run_and_expect_errors, ""}
    }
}

mod prettier {
    mod ok {
        tests_macros::gen_tests! {"test_data/prettier/ok/**/*.{json}", crate::tests::run_and_expect_no_errors, ""}
    }
    mod err {
        tests_macros::gen_tests! {"test_data/prettier/err/**/*.{json}", crate::tests::run_and_expect_errors, ""}
    }
}

fn assert_errors_are_present(program: &Parse<JsonRoot>, path: &Path) {
    assert!(
        !program.diagnostics().is_empty(),
        "There should be errors in the file {:?}\nSyntax Tree: {:#?}",
        path.display(),
        program.syntax()
    );
}

// sometimes our parser emits unknown nodes without diagnostics;
// this check makes sure that we don't signal that the tree has errors.
fn has_unknown_nodes(node: &JsonSyntaxNode) -> bool {
    node.descendants()
        .any(|descendant| descendant.kind().is_unknown())
}

fn assert_errors_are_absent<T>(program: &Parse<T>, path: &Path)
where
    T: AstNode<Language = JsonLanguage> + Debug,
{
    let syntax = program.syntax();
    let debug_tree = format!("{:?}", program.tree());
    let has_missing_children = debug_tree.contains("missing (required)");
    println!("fuck: {}, {}, {}", program.diagnostics().len(), program.has_errors(), has_missing_children);
    if !program.has_errors() && !has_unknown_nodes(&syntax) && !has_missing_children {
        return;
    }
    println!("{}", debug_tree);

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

// #[test]
// pub fn test_trivia_attached_to_tokens() {
//     let text = "/**/let a = 1; // nice variable \n /*hey*/ let \t b = 2; // another nice variable";
//     let m = parse_module(text, 0);
//     let mut tokens = m.syntax().descendants_tokens(Direction::Next);

//     let is_let = |x: &JsSyntaxToken| x.text_trimmed() == "let";
//     let first_let = tokens.find(is_let).unwrap();

//     // first let leading trivia asserts
//     let pieces: Vec<_> = first_let.leading_trivia().pieces().collect();
//     assert!(matches!(pieces.get(0).map(|x| x.text()), Some("/**/")));
//     assert!(matches!(pieces.get(1), None));

//     // first let trailing trivia asserts
//     let pieces: Vec<_> = first_let.trailing_trivia().pieces().collect();
//     assert!(matches!(pieces.get(0).map(|x| x.text()), Some(" ")));
//     assert!(matches!(pieces.get(1), None));

//     // second let leading trivia asserts
//     let second_let = tokens.find(is_let).unwrap();
//     let pieces: Vec<_> = second_let.leading_trivia().pieces().collect();
//     assert_eq!(4, pieces.len());
//     assert!(matches!(pieces.get(0).map(|x| x.text()), Some("\n")));
//     assert!(matches!(pieces.get(1).map(|x| x.text()), Some(" ")));
//     assert!(matches!(pieces.get(2).map(|x| x.text()), Some("/*hey*/")));
//     assert!(matches!(pieces.get(3).map(|x| x.text()), Some(" ")));

//     // second let trailing trivia asserts
//     let pieces: Vec<_> = second_let.trailing_trivia().pieces().collect();
//     assert_eq!(1, pieces.len());
//     assert!(matches!(pieces.get(0).map(|x| x.text()), Some(" \t ")));
// }
