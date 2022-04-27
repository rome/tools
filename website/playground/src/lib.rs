#![allow(clippy::unused_unit)] // Bug in wasm_bindgen creates unused unit warnings. See wasm_bindgen#2774

use rome_diagnostics::file::SimpleFiles;
use rome_diagnostics::termcolor::{ColorSpec, WriteColor};
use rome_diagnostics::Emitter;
use rome_js_formatter::{format_node, FormatOptions, IndentStyle};
use rome_js_parser::{parse, LanguageVariant, SourceType};
use std::io;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RomeOutput {
    ast: String,
    cst: String,
    formatted_code: String,
    formatter_ir: String,
    errors: String,
}

#[wasm_bindgen]
impl RomeOutput {
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
    quote_style: String,
    is_typescript: bool,
    is_jsx: bool,
    source_type: String,
) -> RomeOutput {
    let mut simple_files = SimpleFiles::new();
    let main_file_id = simple_files.add("main.js".to_string(), code.clone());

    let source_type = if source_type == "script" {
        SourceType::js_script()
    } else {
        let source_type = if is_typescript {
            SourceType::ts()
        } else {
            SourceType::js_module()
        };

        if is_jsx {
            source_type.with_variant(LanguageVariant::Jsx)
        } else {
            source_type
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
        line_width: line_width.try_into().unwrap_or_default(),
        quote_style: quote_style.parse().unwrap_or_default(),
    };

    let cst = serde_json::to_string(&syntax)
        .unwrap_or_else(|_| "{ error: \"CST could not be serialized\" }".to_string());
    let ast = serde_json::to_string(&parse.tree())
        .unwrap_or_else(|_| "{ error: \"AST could not be serialized\" }".to_string());
    let formatted = format_node(options, &syntax).unwrap();
    let formatted_code = formatted.print().into_code();

    let root_element = formatted.into_format_element();
    let formatter_ir = format!("{:#?}", root_element);

    let mut errors = ErrorOutput(Vec::new());
    for diag in parse.diagnostics() {
        Emitter::new(&simple_files)
            .emit_with_writer(diag, &mut errors)
            .unwrap();
    }

    RomeOutput {
        cst,
        ast,
        formatted_code,
        formatter_ir,
        errors: String::from_utf8(errors.0).unwrap(),
    }
}
