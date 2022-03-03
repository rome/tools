use rome_formatter::{format as format_code, to_format_element, FormatOptions, IndentStyle};
use rslint_errors::file::SimpleFiles;
use rslint_errors::termcolor::{ColorSpec, WriteColor};
use rslint_errors::{Formatter as ErrorFormatter, LongFormatter};
use rslint_parser::{parse, SourceType};
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
    is_tab: bool,
    indent_width: u8,
    is_typescript: bool,
    is_jsx: bool,
) -> PlaygroundResult {
    let mut simple_files = SimpleFiles::new();
    let main_file_id = simple_files.add("main.js".to_string(), code.clone());

    let source_type = match (is_typescript, is_jsx) {
        (true, true) => SourceType::tsx(),
        (true, false) => SourceType::ts(),
        (false, true) => SourceType::jsx(),
        (false, false) => SourceType::js_module(),
    };

    let parse = parse(&code, main_file_id, source_type);
    let syntax = parse.syntax();

    let indent_style = if is_tab {
        IndentStyle::Tab
    } else {
        IndentStyle::Space(indent_width)
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
        .emit_with_writer(parse.errors(), &simple_files, &mut errors)
        .unwrap();

    PlaygroundResult {
        cst,
        ast,
        formatted_code,
        formatter_ir,
        errors: String::from_utf8(errors.0).unwrap(),
    }
}
