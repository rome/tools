#![allow(clippy::unused_unit)] // Bug in wasm_bindgen creates unused unit warnings. See wasm_bindgen#2774

mod formatter;
mod parser;

use std::io;
use wasm_bindgen::JsValue;

struct ErrorOutput(Vec<Vec<u8>>);

impl ErrorOutput {
    pub fn to_errors(self) -> Box<[JsValue]> {
        let value = self
            .0
            .into_iter()
            .flat_map(|bytes| String::from_utf8(bytes))
            .map(|string| JsValue::from_str(&string))
            .collect::<Vec<JsValue>>();

        value.into_boxed_slice()
    }
}

impl io::Write for ErrorOutput {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut new_message = Vec::new();
        let result = new_message.write(buf)?;
        self.0.push(new_message);
        Ok(result)
    }

    fn flush(&mut self) -> io::Result<()> {
        let iter = self.0.iter_mut();
        for message in iter {
            message.flush()?;
        }
        Ok(())
    }
}

pub use formatter::{format_js, FormatJsOutput, FormatJsParams, FormatOptions};
pub use parser::{parse_js, LanguageOptions, ParseJsParams};
