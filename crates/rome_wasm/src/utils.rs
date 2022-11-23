use std::fmt::Display;

use js_sys::Error;
use wasm_bindgen::prelude::*;

use rome_console::fmt::HTML;
use rome_console::{fmt::Formatter, markup};
use rome_diagnostics::serde::Diagnostic;
use rome_diagnostics::{DiagnosticExt, LineIndexBuf, PrintDiagnostic, SourceCode};

use super::IDiagnostic;

pub(crate) fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct DiagnosticPrinter {
    file_name: String,
    file_source: SourceCode<String, LineIndexBuf>,
    buffer: Vec<u8>,
}

#[wasm_bindgen]
impl DiagnosticPrinter {
    #[wasm_bindgen(constructor)]
    pub fn new(file_name: String, file_source: String) -> Self {
        let line_starts = LineIndexBuf::from_source_text(&file_source);
        Self {
            file_name,
            file_source: SourceCode {
                text: file_source,
                line_starts: Some(line_starts),
            },
            buffer: Vec::new(),
        }
    }

    pub fn print(&mut self, diagnostic: IDiagnostic) -> Result<(), Error> {
        let diag: Diagnostic = diagnostic.into_serde().map_err(into_error)?;
        let err = diag
            .with_file_path(&self.file_name)
            .with_file_source_code(&self.file_source);

        let mut html = HTML(&mut self.buffer);
        Formatter::new(&mut html)
            .write_markup(markup!({ PrintDiagnostic(&err) }))
            .map_err(into_error)?;

        Ok(())
    }

    pub fn finish(self) -> Result<String, Error> {
        String::from_utf8(self.buffer).map_err(into_error)
    }
}

pub(crate) fn into_error<E: Display>(err: E) -> Error {
    Error::new(&err.to_string())
}
