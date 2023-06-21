#![no_main]

mod rome_parse_d_ts;
mod rome_parse_jsx;
mod rome_parse_module;
mod rome_parse_script;
mod rome_parse_tsx;
mod rome_parse_typescript;

use libfuzzer_sys::{fuzz_target, Corpus};

fn do_fuzz(data: &[u8]) -> Corpus {
    let mut keep = Corpus::Reject;
    if let Corpus::Keep = rome_parse_d_ts::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    if let Corpus::Keep = rome_parse_jsx::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    if let Corpus::Keep = rome_parse_module::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    if let Corpus::Keep = rome_parse_script::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    if let Corpus::Keep = rome_parse_tsx::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    if let Corpus::Keep = rome_parse_typescript::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    keep
}

fuzz_target!(|case: &[u8]| -> Corpus { do_fuzz(case) });
