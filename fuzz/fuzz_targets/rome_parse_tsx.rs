#![cfg_attr(not(feature = "rome_all"), no_main)]

#[path = "rome_common.rs"]
mod rome_common;

use libfuzzer_sys::Corpus;
use rome_js_syntax::JsFileSource;

pub fn do_fuzz(case: &[u8]) -> Corpus {
    let parse_type = JsFileSource::tsx();
    rome_common::fuzz_js_parser_with_source_type(case, parse_type)
}

#[cfg(not(feature = "rome_all"))]
libfuzzer_sys::fuzz_target!(|case: &[u8]| -> Corpus { do_fuzz(case) });
