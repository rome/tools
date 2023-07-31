#![cfg_attr(not(feature = "rome_all"), no_main)]

#[path = "rome_common.rs"]
mod rome_common;

use libfuzzer_sys::Corpus;

pub fn do_fuzz(case: &[u8]) -> Corpus {
    rome_common::fuzz_json_parser(case)
}

#[cfg(not(feature = "rome_all"))]
libfuzzer_sys::fuzz_target!(|case: &[u8]| -> Corpus { do_fuzz(case) });
