use rome_formatter_test::check_reformat::CheckReformat;
use rome_json_formatter::context::JsonFormatOptions;
use rome_json_formatter::format_node;
use rome_json_parser::{parse_json, JsonParserConfig};

mod language {
    include!("language.rs");
}

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
{
    "enabled": true,
    "formatWithErrors": false,
    "indentSize": 2,
    "indentStyle": "space",
    "lineWidth": 80,
    "ignore": [
      "**/cache/**",
      "**/dist/**",
      "./packages/laravel/**/*",
      "./packages/presets/templates/**/*",
      "./sandboxes/**/*"
    ]
  }
"#;
    let parse = parse_json(src, JsonParserConfig::default());
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
        r#"{
    "a": 5,
    "b": [1, 2, 3, 4],
    "c": null,
    "d": true,
    "e": false
}
"#
    );
}
