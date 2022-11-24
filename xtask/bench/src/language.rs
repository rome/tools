use crate::test_case::TestCase;
use criterion::black_box;
use rome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleCategories};
use rome_diagnostics::FileId;
use rome_formatter::{FormatResult, Formatted, PrintResult, Printed};
use rome_js_analyze::analyze;
use rome_js_formatter::context::{JsFormatContext, JsFormatOptions};
use rome_js_syntax::{JsAnyRoot, JsSyntaxNode, SourceType};
use rome_parser::prelude::ParseDiagnostic;

pub enum Parse<'a> {
    JavaScript(SourceType, &'a str),
    Json(&'a str),
}

impl<'a> Parse<'a> {
    pub fn try_from_case(case: &TestCase) -> Option<Parse> {
        match SourceType::try_from(case.path()) {
            Ok(source_type) => Some(Parse::JavaScript(source_type, case.code())),
            Err(_) => match case.extension() {
                "json" => Some(Parse::Json(case.code())),
                _ => None,
            },
        }
    }

    pub fn parse(&self) -> Parsed {
        match self {
            Parse::JavaScript(source_type, code) => Parsed::JavaScript(
                rome_js_parser::parse(code, FileId::zero(), *source_type),
                *source_type,
            ),
            Parse::Json(code) => Parsed::Json(rome_json_parser::parse_json(code, FileId::zero())),
        }
    }
}

pub enum Parsed {
    JavaScript(rome_js_parser::Parse<JsAnyRoot>, SourceType),
    Json(rome_json_parser::JsonParse),
}

impl Parsed {
    pub fn format_node(&self) -> Option<FormatNode> {
        match self {
            Parsed::JavaScript(parse, source_type) => {
                Some(FormatNode::JavaScript(parse.syntax(), *source_type))
            }
            Parsed::Json(_) => None,
        }
    }

    pub fn analyze(&self) -> Option<Analyze> {
        match self {
            Parsed::JavaScript(parse, _) => Some(Analyze::JavaScript(parse.tree())),
            Parsed::Json(_) => None,
        }
    }

    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        match self {
            Parsed::JavaScript(parse, _) => parse.into_diagnostics(),
            Parsed::Json(parse) => parse.into_diagnostics(),
        }
    }
}

pub enum FormatNode {
    JavaScript(JsSyntaxNode, SourceType),
}

impl FormatNode {
    pub fn format_node(&self) -> FormatResult<FormattedNode> {
        match self {
            Self::JavaScript(root, source_type) => {
                rome_js_formatter::format_node(JsFormatOptions::new(source_type.clone()), root)
                    .map(FormattedNode::JavaScript)
            }
        }
    }
}

pub enum FormattedNode {
    JavaScript(Formatted<JsFormatContext>),
}

impl FormattedNode {
    pub fn print(&self) -> PrintResult<Printed> {
        match self {
            FormattedNode::JavaScript(formatted) => formatted.print(),
        }
    }
}

pub enum Analyze {
    JavaScript(JsAnyRoot),
}

impl Analyze {
    pub fn analyze(&self) {
        match self {
            Analyze::JavaScript(root) => {
                let filter = AnalysisFilter {
                    categories: RuleCategories::SYNTAX | RuleCategories::LINT,
                    ..AnalysisFilter::default()
                };
                let options = AnalyzerOptions::default();
                analyze(FileId::zero(), root, filter, &options, |event| {
                    black_box(event.diagnostic());
                    black_box(event.actions());
                    ControlFlow::<Never>::Continue(())
                });
            }
        }
    }
}
