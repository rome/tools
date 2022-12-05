use rome_diagnostics::location::FileId;
use rome_formatter::{FormatContext, FormatResult, Formatted, IndentStyle, LineWidth, Printed};
use rome_formatter_test::TestFormatLanguage;
use rome_json_formatter::context::{JsonFormatContext, JsonFormatOptions};
use rome_json_formatter::{format_node, format_range, JsonFormatLanguage};
use rome_json_parser::parse_json;
use rome_json_syntax::JsonLanguage;
use rome_parser::AnyParse;
use rome_rowan::{SyntaxNode, TextRange};
use serde::{Deserialize, Serialize};

pub struct JsonTestFormatLanguage {
    pub options: JsonFormatOptions,
}

impl JsonTestFormatLanguage {
    pub fn new(options: JsonFormatOptions) -> Self {
        JsonTestFormatLanguage { options }
    }
}

impl TestFormatLanguage for JsonTestFormatLanguage {
    type SyntaxLanguage = JsonLanguage;
    type Options = JsonFormatOptions;
    type Context = JsonFormatContext;
    type FormatLanguage = JsonFormatLanguage;

    fn from_format_options(format_options: &Self::Options) -> Self {
        JsonTestFormatLanguage::new(format_options.clone())
    }

    fn parse(&self, text: &str) -> AnyParse {
        parse_json(text, FileId::zero()).into()
    }

    fn format_options(&self) -> Self::Options {
        self.options.clone()
    }

    fn deserialize_format_options(
        &self,
        options: &str,
    ) -> Vec<<Self::Context as FormatContext>::Options> {
        let test_options: TestOptions = serde_json::from_str(options).unwrap();

        test_options
            .cases
            .into_iter()
            .map(|case| case.into())
            .collect()
    }

    fn format_node(
        &self,
        options: Self::Options,
        node: &SyntaxNode<Self::SyntaxLanguage>,
    ) -> FormatResult<Formatted<Self::Context>> {
        format_node(options, node)
    }

    fn format_range(
        &self,
        options: Self::Options,
        node: &SyntaxNode<Self::SyntaxLanguage>,
        range: TextRange,
    ) -> FormatResult<Printed> {
        format_range(options, node, range)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum JsonSerializableIndentStyle {
    /// Tab
    Tab,
    /// Space, with its quantity
    Space(u8),
}

impl From<JsonSerializableIndentStyle> for IndentStyle {
    fn from(test: JsonSerializableIndentStyle) -> Self {
        match test {
            JsonSerializableIndentStyle::Tab => IndentStyle::Tab,
            JsonSerializableIndentStyle::Space(spaces) => IndentStyle::Space(spaces),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct JsonSerializableFormatOptions {
    /// The indent style.
    pub indent_style: Option<JsonSerializableIndentStyle>,

    /// What's the max width of a line. Defaults to 80.
    pub line_width: Option<u16>,
}

impl From<JsonSerializableFormatOptions> for JsonFormatOptions {
    fn from(test: JsonSerializableFormatOptions) -> Self {
        JsonFormatOptions::default()
            .with_indent_style(
                test.indent_style
                    .map_or_else(|| IndentStyle::Tab, |value| value.into()),
            )
            .with_line_width(
                test.line_width
                    .and_then(|width| LineWidth::try_from(width).ok())
                    .unwrap_or_default(),
            )
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestOptions {
    cases: Vec<JsonSerializableFormatOptions>,
}
