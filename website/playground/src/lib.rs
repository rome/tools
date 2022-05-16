#![allow(clippy::unused_unit)] // Bug in wasm_bindgen creates unused unit warnings. See wasm_bindgen#2774

use rome_analyze::AnalysisFilter;
use rome_diagnostics::file::SimpleFiles;
use rome_diagnostics::termcolor::{ColorSpec, WriteColor};
use rome_diagnostics::Emitter;
use rome_formatter::{FormatOptions, IndentStyle};
use rome_js_formatter::format_node;
use rome_js_parser::{parse, LanguageVariant, SourceType};
use serde_json::json;
use std::io;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

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

/// Serde's default serialization results in a lot of nesting because of how it serializes
/// Results and Vectors. We flatten this nesting to make the JSON easier to read
fn clean_up_json(json: serde_json::Value) -> serde_json::Value {
    match json {
        serde_json::Value::Array(entries) => {
            serde_json::Value::Array(entries.into_iter().map(clean_up_json).collect())
        }
        serde_json::Value::Object(mut fields) => {
            if fields.len() == 1 && fields.contains_key("Ok") {
                clean_up_json(fields.remove("Ok").unwrap())
            } else {
                serde_json::Value::Object(
                    fields
                        .into_iter()
                        .map(|(key, value)| (key, clean_up_json(value)))
                        .collect(),
                )
            }
        }
        s => s,
    }
}
#[wasm_bindgen]
pub struct PlaygroundFormatOptions {
    line_width: u16,
    indent_width: Option<u8>, // If None, we use tabs
    quote_style: String,
}

#[wasm_bindgen]
impl PlaygroundFormatOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        line_width: u16,
        indent_width: Option<u8>, // If None, we use tabs
        quote_style: String,
    ) -> Self {
        Self {
            line_width,
            indent_width,
            quote_style,
        }
    }
}

#[wasm_bindgen]
pub fn run(
    code: String,
    options: PlaygroundFormatOptions,
    is_typescript: bool,
    is_jsx: bool,
    source_type: String,
    output_json: bool,
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

    let indent_style = if let Some(width) = options.indent_width {
        IndentStyle::Space(width)
    } else {
        IndentStyle::Tab
    };

    let options = FormatOptions {
        indent_style,
        line_width: options.line_width.try_into().unwrap_or_default(),
        quote_style: options.quote_style.parse().unwrap_or_default(),
    };

    let (cst, ast) = if output_json {
        let cst_json = clean_up_json(
            serde_json::to_value(&syntax)
                .unwrap_or_else(|_| json!({ "error": "CST could not be serialized" })),
        );

        let ast_json = clean_up_json(
            serde_json::to_value(&parse.tree())
                .unwrap_or_else(|_| json!({ "error": "AST could not be serialized" })),
        );

        (cst_json.to_string(), ast_json.to_string())
    } else {
        (format!("{:#?}", syntax), format!("{:#?}", parse.tree()))
    };

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

    rome_analyze::analyze(
        main_file_id,
        &parse.tree(),
        AnalysisFilter::default(),
        |signal| {
            if let Some(mut diag) = signal.diagnostic() {
                if let Some(action) = signal.action() {
                    diag.suggestions.push(action.into());
                }

                Emitter::new(&simple_files)
                    .emit_with_writer(&diag, &mut errors)
                    .unwrap();
            }
        },
    );

    RomeOutput {
        cst,
        ast,
        formatted_code,
        formatter_ir,
        errors: String::from_utf8(errors.0).unwrap(),
    }
}
