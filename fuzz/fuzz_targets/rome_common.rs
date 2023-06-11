#![allow(dead_code)]

use libfuzzer_sys::Corpus;
use rome_js_parser::parse;
use rome_js_syntax::JsFileSource;
use rome_json_parser::parse_json;

pub fn fuzz_js_parser_with_source_type(data: &[u8], source: JsFileSource) -> Corpus {
    let Ok(code1) = std::str::from_utf8(data) else { return Corpus::Reject; };

    let parse1 = parse(code1, source);
    if !parse1.has_errors() {
        let syntax1 = parse1.syntax();
        let code2 = syntax1.to_string();
        assert_eq!(code1, code2, "unparse output differed");
    }

    Corpus::Keep
}

pub fn fuzz_json_parser(data: &[u8]) -> Corpus {
    let Ok(code1) = std::str::from_utf8(data) else { return Corpus::Reject; };

    let parse1 = parse_json(code1);
    if !parse1.has_errors() {
        let syntax1 = parse1.syntax();
        let code2 = syntax1.to_string();
        assert_eq!(code1, code2, "unparse output differed");
    }

    Corpus::Keep
}
