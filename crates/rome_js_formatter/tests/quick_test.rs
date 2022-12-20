use rome_diagnostics::FileId;
use rome_formatter_test::check_reformat::CheckReformat;
use rome_js_formatter::context::{JsFormatOptions, Semicolons};
use rome_js_formatter::format_node;
use rome_js_parser::parse;
use rome_js_syntax::SourceType;

mod language {
    include!("language.rs");
}

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
f<number> delete;
"#;
    let syntax = SourceType::tsx();
    let tree = parse(src, FileId::zero(), syntax);
    let options = JsFormatOptions::new(syntax).with_semicolons(Semicolons::AsNeeded);
    let result = format_node(options.clone(), &tree.syntax())
        .unwrap()
        .print()
        .unwrap();

    let root = &tree.syntax();
    let language = language::JsTestFormatLanguage::new(SourceType::tsx());
    let check_reformat =
        CheckReformat::new(root, result.as_code(), "quick_test", &language, options);
    check_reformat.check_reformat();

    assert_eq!(
        result.as_code(),
        r#"[
	5,
	7234932436,
    // comment 3
];
"#
    );
}
