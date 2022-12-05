use rome_diagnostics::location::FileId;
use rome_formatter::{FormatLanguage, IndentStyle, LineWidth};
use rome_formatter_test::TestFormatLanguage;
use rome_fs::RomePath;
use rome_json_formatter::context::JsonFormatOptions;
use rome_json_formatter::JsonFormatLanguage;
use rome_json_parser::parse_json;
use rome_parser::AnyParse;
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
    type FormatLanguage = JsonFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        parse_json(text, FileId::zero()).into()
    }

    fn format_language(&self) -> Self::FormatLanguage {
        JsonFormatLanguage::new(self.options.clone())
    }

    fn read_format_languages_from_file(&self, path: &mut RomePath) -> Vec<Self::FormatLanguage> {
        let test_options: TestOptions =
            serde_json::from_str(path.get_buffer_from_file().as_str()).unwrap();

        test_options
            .cases
            .into_iter()
            .map(|case| JsonFormatLanguage::new(case.into()))
            .collect()
    }

    fn from_format_language(format_language: &Self::FormatLanguage) -> Self {
        JsonTestFormatLanguage::new(format_language.options().clone())
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
