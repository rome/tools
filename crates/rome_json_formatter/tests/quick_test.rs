use rome_formatter_test::check_reformat::CheckReformat;
use rome_json_formatter::context::JsonFormatOptions;
use rome_json_formatter::format_node;
use rome_json_parser::{parse_json, JsonParserOptions};

mod language {
    include!("language.rs");
}

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"// comment
 // comment
 { "test": "test"}
"#;
    let parse = parse_json(src, JsonParserOptions::default().with_allow_comments());
    let options = JsonFormatOptions::default();
    let result = format_node(options.clone(), &parse.syntax())
        .unwrap()
        .print()
        .unwrap();

    let root = &parse.syntax();
    let language = language::JsonTestFormatLanguage::default();

    let check_reformat =
        CheckReformat::new(root, result.as_code(), "quick_test", &language, options);
    check_reformat.check_reformat();

    assert_eq!(
        result.as_code(),
        r#"// comment
// comment
{ "test": "test"}
"#
    );
}
