//! Common functionality between different fuzzers. Look here if you need to inspect implementation
//! details for the fuzzer harnesses!

#![allow(dead_code)]

use libfuzzer_sys::Corpus;
use rome_formatter::format_node;
use rome_js_formatter::context::JsFormatOptions;
use rome_js_formatter::JsFormatLanguage;
use rome_js_parser::parse;
use rome_js_syntax::JsFileSource;
use rome_json_formatter::context::JsonFormatOptions;
use rome_json_formatter::JsonFormatLanguage;
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

pub fn fuzz_js_formatter_with_source_type(data: &[u8], source: JsFileSource) -> Corpus {
    let Ok(code1) = std::str::from_utf8(data) else { return Corpus::Reject; };

    let parse1 = parse(code1, source);
    if !parse1.has_errors() {
        let language = JsFormatLanguage::new(JsFormatOptions::new(source));
        let syntax1 = parse1.syntax();
        if let Ok(formatted1) = format_node(&syntax1, language.clone()) {
            if let Ok(printed1) = formatted1.print() {
                let code2 = printed1.as_code();
                let parse2 = parse(code2, source);
                assert!(!parse2.has_errors(), "formatter introduced errors");
                let syntax2 = parse2.syntax();
                let formatted2 = format_node(&syntax2, language)
                    .expect("formatted code could not be reformatted");
                let printed2 = formatted2
                    .print()
                    .expect("reformatted code could not be printed");
                assert_eq!(code2, printed2.as_code(), "format results differ")
            }
        }
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

pub fn fuzz_json_formatter(data: &[u8]) -> Corpus {
    let Ok(code1) = std::str::from_utf8(data) else { return Corpus::Reject; };

    let parse1 = parse_json(code1);
    if !parse1.has_errors() {
        let language = JsonFormatLanguage::new(JsonFormatOptions::default());
        let syntax1 = parse1.syntax();
        if let Ok(formatted1) = format_node(&syntax1, language.clone()) {
            if let Ok(printed1) = formatted1.print() {
                let code2 = printed1.as_code();
                let parse2 = parse_json(code2);
                assert!(!parse2.has_errors(), "formatter introduced errors");
                let syntax2 = parse2.syntax();
                let formatted2 = format_node(&syntax2, language)
                    .expect("formatted code could not be reformatted");
                let printed2 = formatted2
                    .print()
                    .expect("reformatted code could not be printed");
                assert_eq!(code2, printed2.as_code(), "format results differ")
            }
        }
    }

    Corpus::Keep
}
