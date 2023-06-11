use libfuzzer_sys::Corpus;
use rome_js_parser::parse;
use rome_js_syntax::JsFileSource;

pub fn fuzz_source_type(data: &[u8], source: JsFileSource) -> Corpus {
    let Ok(code1) = std::str::from_utf8(data) else { return Corpus::Reject; };

    let parse1 = parse(code1, source);
    if !parse1.has_errors() {
        let code2 = parse1.tree().to_string();
        let parse2 = parse(&code2, source);
        assert!(
            !parse2.has_errors(),
            "unparsing introduced a formatting error"
        );
        assert_eq!(code2, parse2.tree().to_string());
    }

    Corpus::Keep
}
