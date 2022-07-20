use crate::parser::{ParseJsParams, _internal_parse_js};
use crate::{ErrorOutput, LanguageOptions};
use rome_diagnostics::file::SimpleFiles;
use rome_diagnostics::Emitter;
use rome_formatter::IndentStyle;
use rome_js_formatter::context::JsFormatContext;
use rome_js_formatter::format_node;
use rome_js_syntax::SourceType;
use wasm_bindgen::prelude::*;

#[derive(Default)]
#[wasm_bindgen]
pub struct FormatOptions {
    line_width: Option<u16>,
    indent_width: Option<u8>, // If None, we use tabs
    quote_style: Option<String>,
    /// If true, you'll be able to read the IR
    debug: bool,
}

impl FormatOptions {
    fn into_context(self, source_type: SourceType) -> JsFormatContext {
        let indent_style = self
            .indent_width
            .map(|width| IndentStyle::Space(width))
            .unwrap_or(IndentStyle::Tab);

        let context = JsFormatContext::new(source_type)
            .with_indent_style(indent_style)
            .with_line_width(
                self.line_width
                    .and_then(|l| l.try_into().ok())
                    .unwrap_or_default(),
            )
            .with_quote_style(
                self.quote_style
                    .and_then(|q| q.parse().ok())
                    .unwrap_or_default(),
            );

        context
    }
}

#[wasm_bindgen]
impl FormatOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        line_width: Option<u16>,
        indent_width: Option<u8>,
        quote_style: Option<String>,
        debug: bool,
    ) -> Self {
        Self {
            line_width,
            indent_width,
            quote_style,
            debug,
        }
    }
}

#[wasm_bindgen]
pub struct FormatJsParams {
    format_options: Option<FormatOptions>,
    language_options: Option<LanguageOptions>,
}

#[wasm_bindgen]
impl FormatJsParams {
    #[wasm_bindgen(constructor)]
    pub fn new(
        format_options: Option<FormatOptions>,
        language_options: Option<LanguageOptions>,
    ) -> Self {
        Self {
            format_options,
            language_options,
        }
    }
}

#[wasm_bindgen]
pub struct FormatJsOutput {
    formatted_code: String,
    errors: Box<[JsValue]>,
    formatted_ir: Option<String>,
}

#[wasm_bindgen]
impl FormatJsOutput {
    #[wasm_bindgen(js_name = "formattedCode")]
    pub fn formatted_code(&self) -> String {
        String::from(self.formatted_code.clone())
    }

    #[wasm_bindgen]
    pub fn errors(&self) -> Box<[JsValue]> {
        self.errors.clone()
    }

    #[wasm_bindgen(js_name = "ir")]
    pub fn ir(&self) -> Option<String> {
        if let Some(ir) = &self.formatted_ir {
            Some(String::from(ir.clone()))
        } else {
            None
        }
    }
}

/// Formats some JavaScript/TypeScript/JSX code
#[wasm_bindgen(js_name = formatJs)]
pub fn format_js(code: &str, params: FormatJsParams) -> FormatJsOutput {
    let FormatJsParams {
        format_options,
        language_options,
    } = params;

    let format_debug = format_options
        .as_ref()
        .map(|options| options.debug)
        .unwrap_or_default();
    let mut simple_files = SimpleFiles::new();
    let content = String::from(code.clone());
    let main_file_id = simple_files.add("main.js".to_string(), content);

    let source_type = if let Some(language_options) = language_options {
        language_options.source_type()
    } else {
        SourceType::js_module()
    };
    let parsed_code = _internal_parse_js(
        code,
        main_file_id,
        ParseJsParams {
            language_options,
            debug: format_debug,
        },
    );

    let errors = ErrorOutput(Vec::new());
    for diag in parsed_code.diagnostics() {
        Emitter::new(&simple_files)
            .emit_stderr(diag, false)
            .unwrap();
    }

    let context = if let Some(format_options) = format_options {
        format_options.into_context(source_type)
    } else {
        JsFormatContext::default()
    };

    let syntax = parsed_code.syntax();
    let formatted = format_node(context, &syntax).unwrap();
    let formatted_code = formatted.print().into_code();

    let formatted_ir = if format_debug {
        let root_element = formatted.into_format_element();
        Some(format!("{:#?}", root_element))
    } else {
        None
    };

    FormatJsOutput {
        formatted_code,
        errors: errors.to_errors(),
        formatted_ir,
    }
}
