#![no_main]

mod rome_format_d_ts;
mod rome_format_jsx;
mod rome_format_module;
mod rome_format_script;
mod rome_format_tsx;
mod rome_format_typescript;

use libfuzzer_sys::{fuzz_target, Corpus};

fn do_fuzz(data: &[u8]) -> Corpus {
    let mut keep = Corpus::Reject;
    if let Corpus::Keep = rome_format_d_ts::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    if let Corpus::Keep = rome_format_jsx::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    if let Corpus::Keep = rome_format_module::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    if let Corpus::Keep = rome_format_script::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    if let Corpus::Keep = rome_format_tsx::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    if let Corpus::Keep = rome_format_typescript::do_fuzz(data) {
        keep = Corpus::Keep;
    }
    keep
}

fuzz_target!(|case: &[u8]| -> Corpus { do_fuzz(case) });
