use super::IDiagnostic;
use js_sys::Error;
use rome_console::fmt::{Termcolor, HTML};
use rome_console::{fmt::Formatter, markup};
use rome_diagnostics::file::SimpleFile;
use rome_diagnostics::termcolor::NoColor;
use rome_diagnostics::Diagnostic;
use std::fmt::Display;
use wasm_bindgen::prelude::*;

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
    file: SimpleFile,
    buffer: Vec<Vec<u8>>,
    printer_kind: PrinterKind,
}

// TODO: wasm_bindgen doesn't support enum with values, but only C-style enums.
// Check progress on PR: https://github.com/rustwasm/wasm-bindgen/pull/2631
// Issue: https://github.com/rustwasm/wasm-bindgen/issues/2407
#[wasm_bindgen]
/// How to print diagnostics
pub enum PrinterKind {
    /// Diagnostics are printed for HTML usage
    Html = 1,
    /// Diagnostics are printed for terminal usage
    Terminal = 2,
}

#[wasm_bindgen]
impl DiagnosticPrinter {
    #[wasm_bindgen(constructor)]
    /// It creates a new diagnostic printer for the current file
    pub fn new(file_name: String, file_source: String, printer_kind: PrinterKind) -> Self {
        Self {
            file: SimpleFile::new(file_name, file_source),
            buffer: Vec::default(),
            printer_kind,
        }
    }

    pub fn print(&mut self, diagnostic: IDiagnostic) -> Result<(), Error> {
        let diag: Diagnostic = diagnostic.into_serde().map_err(into_error)?;

        let mut buff = Vec::new();

        match self.printer_kind {
            PrinterKind::Html => {
                let mut html = HTML(&mut buff);
                Formatter::new(&mut html)
                    .write_markup(markup!({ diag.display(&self.file) }))
                    .map_err(into_error)?;
            }
            PrinterKind::Terminal => {
                let mut terminal = Termcolor(NoColor::new(&mut buff));
                Formatter::new(&mut terminal)
                    .write_markup(markup!({ diag.display(&self.file) }))
                    .map_err(into_error)?;
            }
        };

        self.buffer.push(buff);

        Ok(())
    }

    /// It returns all diagnostics printed into a single string
    pub fn finish(self) -> Result<String, Error> {
        let flat_buffer: Vec<_> = self.buffer.into_iter().flat_map(|b| b).collect();
        String::from_utf8(flat_buffer).map_err(into_error)
    }

    /// It returns an array, where each item is diagnostics printed into a string
    pub fn finish_as_separated(self) -> Result<JsValue, Error> {
        let mut result = Vec::with_capacity(self.buffer.len());
        for data in self.buffer.into_iter() {
            let as_string = String::from_utf8(data).map_err(into_error)?;
            result.push(as_string);
        }
        JsValue::from_serde(&result).map_err(into_error)
    }
}

pub(crate) fn into_error<E: Display>(err: E) -> Error {
    Error::new(&err.to_string())
}
