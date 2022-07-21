#![allow(clippy::unused_unit)] // Bug in wasm_bindgen creates unused unit warnings. See wasm_bindgen#2774

use js_sys::Array;
use rome_analyze::{AnalysisFilter, ControlFlow, Never};
use rome_console::fmt::{Formatter, HTML};
use rome_console::{markup, Markup};
use rome_diagnostics::file::SimpleFiles;
use rome_diagnostics::Diagnostic;
use rome_formatter::IndentStyle;
use rome_js_formatter::context::JsFormatContext;
use rome_js_formatter::format_node;
use rome_js_parser::parse;
use rome_js_syntax::{LanguageVariant, SourceType};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = performance)]
    fn mark(name: &str);

    #[wasm_bindgen(js_namespace = performance)]
    fn measure(name: &str, begin: &str, end: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = performance)]
    fn getEntriesByName(name: &str, ty: &str) -> Array;
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

fn measure_and_print(name: &str, begin: &str, end: &str) {
    let parse_measure = measure(name, begin, end);
    // Firefox returns undefined
    if parse_measure.is_truthy() {
        if let Ok(parse_measure) = js_sys::JSON::stringify(&parse_measure) {
            log(&parse_measure.as_string().unwrap());
        }
    } else {
        let entries = getEntriesByName(name, "measure");
        if entries.is_truthy() {
            let entry = entries.get(0);
            if entry.is_truthy() {
                let json = js_sys::JSON::stringify(&entry).unwrap();
                log(&json.as_string().unwrap());
            }
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

    mark("rome::begin");
    mark("rome::parse::begin");
    let parse = parse(&code, main_file_id, source_type);
    mark("rome::parse::end");
    measure_and_print("rome::parse", "rome::parse::begin", "rome::parse::end");

    let syntax = parse.syntax();

    let indent_style = if let Some(width) = options.indent_width {
        IndentStyle::Space(width)
    } else {
        IndentStyle::Tab
    };

    let context = JsFormatContext::new(source_type)
        .with_indent_style(indent_style)
        .with_line_width(options.line_width.try_into().unwrap_or_default())
        .with_quote_style(options.quote_style.parse().unwrap_or_default());

    let (cst, ast) = (format!("{:#?}", syntax), format!("{:#?}", parse.tree()));

    mark("rome::format::begin");
    let formatted = format_node(context, &syntax).unwrap();
    let formatted_code = formatted.print().into_code();
    mark("rome::format::end");
    measure_and_print("rome::format", "rome::format::begin", "rome::format::end");

    let root_element = formatted.into_format_element();
    let formatter_ir = format!("{:#?}", root_element);

    let mut html = HTML(Vec::new());
    for diag in parse.diagnostics() {
        diagnostic_to_string(&simple_files, main_file_id, diag, &mut html);
    }

    mark("rome::analyze::begin");
    rome_js_analyze::analyze(
        main_file_id,
        &parse.tree(),
        AnalysisFilter::default(),
        |signal| {
            if let Some(mut diag) = signal.diagnostic() {
                if let Some(action) = signal.action() {
                    diag.suggestions.push(action.into());
                }
                diagnostic_to_string(&simple_files, main_file_id, &diag, &mut html);
            }

            ControlFlow::<Never>::Continue(())
        },
    );
    mark("rome::analyze::end");
    measure_and_print(
        "rome::analyze",
        "rome::analyze::begin",
        "rome::analyze::end",
    );

    mark("rome::end");
    measure_and_print("rome", "rome::begin", "rome::end");

    // Make easier to read each set of mearures
    log("");

    RomeOutput {
        cst,
        ast,
        formatted_code,
        formatter_ir,
        errors: String::from_utf8(html.0).unwrap(),
    }
}

fn markup_to_string(markup: Markup, html: &mut HTML<Vec<u8>>) {
    let mut fmt = Formatter::new(html);
    fmt.write_markup(markup).unwrap();
}

fn diagnostic_to_string(
    simple_files: &SimpleFiles,
    id: usize,
    diag: &Diagnostic,
    html: &mut HTML<Vec<u8>>,
) {
    let file = simple_files.get(id).unwrap();
    markup_to_string(
        markup! {
            {diag.display(file)}
        },
        html,
    );
}
