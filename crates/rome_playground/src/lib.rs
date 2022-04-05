#![allow(clippy::unused_unit)] // Bug in wasm_bindgen creates unused unit warnings. See wasm_bindgen#2774

use rome_diagnostics::file::SimpleFiles;
use rome_diagnostics::termcolor::{ColorSpec, WriteColor};
use rome_diagnostics::{Formatter as ErrorFormatter, LongFormatter};
use rome_js_formatter::{format as format_code, to_format_element, FormatOptions, IndentStyle};
use rome_js_parser::{parse, SourceType};
use std::io;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PlaygroundResult {
    ast: String,
    cst: String,
    formatted_code: String,
    formatter_ir: String,
    errors: String,
}

#[wasm_bindgen]
impl PlaygroundResult {
    #[wasm_bindgen(getter)]
    pub fn ast(&self) -> String {
        self.ast.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn cst(&self) -> String {
        self.cst.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn formatted_code(&self) -> String {
        self.formatted_code.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn formatter_ir(&self) -> String {
        self.formatter_ir.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn errors(&self) -> String {
        self.errors.clone()
    }
}

struct ErrorOutput(Vec<u8>);

impl io::Write for ErrorOutput {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl WriteColor for ErrorOutput {
    fn supports_color(&self) -> bool {
        false
    }

    fn set_color(&mut self, _spec: &ColorSpec) -> io::Result<()> {
        Ok(())
    }

    fn reset(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[wasm_bindgen]
pub fn run(
    code: String,
    line_width: u16,
    indent_width: Option<u8>, // If None, we use tabs
    is_typescript: bool,
    is_jsx: bool,
    source_type: String,
) -> PlaygroundResult {
    let mut simple_files = SimpleFiles::new();
    let main_file_id = simple_files.add("main.js".to_string(), code.clone());

    let source_type = if source_type == "script" {
        SourceType::js_script()
    } else {
        match (is_typescript, is_jsx) {
            (true, true) => SourceType::tsx(),
            (true, false) => SourceType::ts(),
            (false, true) => SourceType::jsx(),
            (false, false) => SourceType::js_module(),
        }
    };

    let parse = parse(&code, main_file_id, source_type);
    let syntax = parse.syntax();

    let indent_style = if let Some(width) = indent_width {
        IndentStyle::Space(width)
    } else {
        IndentStyle::Tab
    };

    let options = FormatOptions {
        indent_style,
        line_width,
    };

    let cst = format!("{:#?}", syntax);
    let ast = format!("{:#?}", parse.tree());
    let formatted_code = format_code(options, &syntax)
        // TODO: impl Error for FormatError
        .unwrap()
        .into_code();

    let root_element = to_format_element(options, &syntax).unwrap();
    let formatter_ir = format!("{:#?}", root_element);

    let mut errors = ErrorOutput(Vec::new());
    let mut error_formatter = LongFormatter;
    error_formatter
        .emit_with_writer(parse.diagnostics(), &simple_files, &mut errors)
        .unwrap();

    PlaygroundResult {
        cst,
        ast,
        formatted_code,
        formatter_ir,
        errors: String::from_utf8(errors.0).unwrap(),
    }
}
