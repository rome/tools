use crate::ErrorOutput;
use rome_diagnostics::file::SimpleFiles;
use rome_diagnostics::Emitter;
use rome_js_parser::{parse, Parse};
use rome_js_syntax::{JsAnyRoot, Language, LanguageVariant, ModuleKind, SourceType};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ParseJsParams {
    pub language_options: Option<LanguageOptions>,
    /// If true, you'll be able to read the CST
    pub(crate) debug: bool,
}

#[wasm_bindgen]
impl ParseJsParams {
    #[wasm_bindgen(constructor)]
    pub fn new(language_options: Option<LanguageOptions>, debug: bool) -> Self {
        Self {
            language_options,
            debug,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct LanguageOptions {
    module_kind: Option<ModuleKind>,
    language_variant: Option<LanguageVariant>,
    language: Option<Language>,
}

#[wasm_bindgen]
pub struct ParseJsOutput {
    cst: Option<String>,
    ast: String,
    errors: Box<[JsValue]>,
}

#[wasm_bindgen]
impl ParseJsOutput {
    #[wasm_bindgen]
    pub fn cst(&self) -> Option<String> {
        if let Some(cst) = &self.cst {
            Some(String::from(cst.clone()))
        } else {
            None
        }
    }

    #[wasm_bindgen]
    pub fn ast(&self) -> String {
        String::from(self.ast.clone())
    }

    #[wasm_bindgen]
    pub fn errors(&self) -> Box<[JsValue]> {
        self.errors.clone()
    }
}

impl LanguageOptions {
    pub fn is_typescript(&self) -> bool {
        matches!(self.language, Some(Language::TypeScript { .. }))
    }

    pub fn is_jsx(&self) -> bool {
        matches!(self.language_variant, Some(LanguageVariant::Jsx))
    }

    pub fn source_type(&self) -> SourceType {
        if self.module_kind == Some(ModuleKind::Script) {
            SourceType::js_script()
        } else {
            let source_type = if self.is_typescript() {
                SourceType::ts()
            } else {
                SourceType::js_module()
            };

            if self.is_jsx() {
                source_type.with_variant(LanguageVariant::Jsx)
            } else {
                source_type
            }
        }
    }
}

/// Internal function to parse some JS code
pub fn _internal_parse_js(code: &str, file_id: usize, params: ParseJsParams) -> Parse<JsAnyRoot> {
    let ParseJsParams {
        language_options,
        debug: _,
    } = params;

    let source_type = if let Some(language_options) = language_options {
        language_options.source_type()
    } else {
        SourceType::js_module()
    };

    parse(&code, file_id, source_type)
}

/// Parses some JavaScript code
#[wasm_bindgen(js_name = "parseJs")]
pub fn parse_js(code: &str, params: ParseJsParams) -> ParseJsOutput {
    let debug = params.debug;
    let mut simple_files = SimpleFiles::new();
    let content = String::from(code.clone());
    let main_file_id = simple_files.add("main.js".to_string(), content);

    let result = _internal_parse_js(code, main_file_id, params);
    let ast = format!("{:#?}", result.syntax());
    let cst = if debug {
        Some(format!("{:#?}", result.tree()))
    } else {
        None
    };

    let errors = ErrorOutput(Vec::new());
    for diag in result.diagnostics() {
        Emitter::new(&simple_files)
            .emit_stderr(diag, false)
            .unwrap();
    }

    ParseJsOutput {
        ast,
        cst,
        errors: errors.to_errors(),
    }
}
