use rome_diagnostics::file::FileId;
use rome_formatter::{FormatOptions, LineWidth};
use rome_formatter::{IndentStyle, Printed};
use rome_fs::RomePath;
use rome_js_formatter::context::{JsFormatOptions, QuoteProperties, QuoteStyle};
use rome_js_formatter::format_node;
use rome_js_parser::parse;
use rome_js_syntax::{ModuleKind, SourceType};
use rome_service::workspace::{FeatureName, SupportsFeatureParams};
use rome_service::App;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

mod check_reformat {
    include!("check_reformat.rs");
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum SerializableIndentStyle {
    /// Tab
    Tab,
    /// Space, with its quantity
    Space(u8),
}

impl From<SerializableIndentStyle> for IndentStyle {
    fn from(test: SerializableIndentStyle) -> Self {
        match test {
            SerializableIndentStyle::Tab => IndentStyle::Tab,
            SerializableIndentStyle::Space(spaces) => IndentStyle::Space(spaces),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum SerializableQuoteStyle {
    Double,
    Single,
}

impl From<SerializableQuoteStyle> for QuoteStyle {
    fn from(test: SerializableQuoteStyle) -> Self {
        match test {
            SerializableQuoteStyle::Double => QuoteStyle::Double,
            SerializableQuoteStyle::Single => QuoteStyle::Single,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum SerializableQuoteProperties {
    AsNeeded,
    Preserve,
}

impl From<SerializableQuoteProperties> for QuoteProperties {
    fn from(test: SerializableQuoteProperties) -> Self {
        match test {
            SerializableQuoteProperties::AsNeeded => QuoteProperties::AsNeeded,
            SerializableQuoteProperties::Preserve => QuoteProperties::Preserve,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct SerializableFormatOptions {
    /// The indent style.
    pub indent_style: Option<SerializableIndentStyle>,

    /// What's the max width of a line. Defaults to 80.
    pub line_width: Option<u16>,

    // The style for quotes. Defaults to double.
    pub quote_style: Option<SerializableQuoteStyle>,

    /// When properties in objects are quoted. Defaults to as-needed.
    pub quote_properties: Option<SerializableQuoteProperties>,
}

impl From<SerializableFormatOptions> for JsFormatOptions {
    fn from(test: SerializableFormatOptions) -> Self {
        Self::new(SourceType::default())
            .with_indent_style(
                test.indent_style
                    .map_or_else(|| IndentStyle::Tab, |value| value.into()),
            )
            .with_line_width(
                test.line_width
                    .and_then(|width| LineWidth::try_from(width).ok())
                    .unwrap_or_default(),
            )
            .with_quote_style(
                test.quote_style
                    .map_or_else(|| QuoteStyle::Double, |value| value.into()),
            )
            .with_quote_properties(
                test.quote_properties
                    .map_or_else(|| QuoteProperties::AsNeeded, |value| value.into()),
            )
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestOptions {
    cases: Vec<SerializableFormatOptions>,
}

#[derive(Debug, Default)]
struct SnapshotContent {
    input: String,
    output: Vec<(String, JsFormatOptions)>,
}

impl SnapshotContent {
    fn add_output(&mut self, formatted: Printed, options: JsFormatOptions) {
        let code = formatted.as_code();
        let mut output: String = code.to_string();
        if !formatted.verbatim_ranges().is_empty() {
            output.push_str("\n\n");
            output.push_str("## Unimplemented nodes/tokens");
            output.push_str("\n\n");
            for (range, text) in formatted.verbatim() {
                writeln!(output, "{:?} => {:?}", text, range).unwrap();
            }
        }

        let line_width_limit = options.line_width().value() as usize;
        let mut exceeding_lines = code
            .lines()
            .enumerate()
            .filter(|(_, line)| line.len() > line_width_limit)
            .peekable();

        if exceeding_lines.peek().is_some() {
            write!(
                output,
                "\n\n## Lines exceeding width of {line_width_limit} characters\n\n"
            )
            .unwrap();

            for (line_index, text) in exceeding_lines {
                let line_number = line_index + 1;
                writeln!(output, "{line_number:>5}: {text}").unwrap();
            }
        }

        self.output.push((output, options));
    }

    fn set_input(&mut self, content: impl Into<String>) {
        self.input = content.into();
    }

    fn snap_content(&mut self) -> String {
        let mut output = String::new();
        output.push_str("# Input");
        output.push('\n');
        output.push_str(self.input.as_str());
        output.push_str("\n=============================\n");

        output.push_str("# Outputs\n");
        let iter = self.output.iter();
        for (index, (content, options)) in iter.enumerate() {
            let formal_index = index + 1;
            writeln!(output, "## Output {formal_index}").unwrap();
            output.push_str("-----\n");
            write!(output, "{}", options).unwrap();
            output.push_str("-----\n");
            output.push_str(content.as_str());
        }

        output
    }
}

/// [insta.rs](https://insta.rs/docs) snapshot testing
///
/// For better development workflow, run
/// `cargo watch -i '*.new' -x 'test -p rome_js_formatter formatter'`
///
/// To review and commit the snapshots, `cargo install cargo-insta`, and run
/// `cargo insta review` or `cargo insta accept`
///
/// The input and the expected output are stored as dedicated files in the `tests/specs` directory where
/// the input file name is `{spec_name}.json` and the output file name is `{spec_name}.json.snap`.
///
/// Specs can be grouped in directories by specifying the directory name in the spec name. Examples:
///
/// # Examples
///
/// * `json/null` -> input: `tests/specs/json/null.json`, expected output: `tests/specs/json/null.json.snap`
/// * `null` -> input: `tests/specs/null.json`, expected output: `tests/specs/null.json.snap`
pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, file_type: &str) {
    let app = App::default();

    let file_path = &spec_input_file;
    let spec_input_file = Path::new(spec_input_file);

    assert!(
        spec_input_file.is_file(),
        "The input '{}' must exist and be a file.",
        spec_input_file.display()
    );

    let mut rome_path = RomePath::new(file_path, FileId::zero());
    let can_format = app
        .workspace
        .supports_feature(SupportsFeatureParams {
            path: rome_path.clone(),
            feature: FeatureName::Format,
        })
        .unwrap();

    if can_format.reason.is_none() {
        let mut snapshot_content = SnapshotContent::default();
        let buffer = rome_path.get_buffer_from_file();
        let mut source_type: SourceType = rome_path.as_path().try_into().unwrap();
        if file_type != "module" {
            source_type = source_type.with_module_kind(ModuleKind::Script);
        }

        let input = fs::read_to_string(file_path).unwrap();
        snapshot_content.set_input(input.as_str());

        let parsed = parse(buffer.as_str(), FileId::zero(), source_type);
        let has_errors = parsed.has_errors();
        let root = parsed.syntax();

        // we ignore the error for now
        let options = JsFormatOptions::new(source_type);
        let formatted = format_node(options.clone(), &root).unwrap();
        let printed = formatted.print().unwrap();
        let file_name = spec_input_file.file_name().unwrap().to_str().unwrap();

        if !has_errors {
            check_reformat::check_reformat(check_reformat::CheckReformatParams {
                root: &root,
                text: printed.as_code(),
                source_type,
                file_name,
                options: options.clone(),
            });
        }

        snapshot_content.add_output(printed, options);

        let test_directory = PathBuf::from(test_directory);
        let options_path = test_directory.join("options.json");
        if options_path.exists() {
            {
                let mut options_path = RomePath::new(&options_path, FileId::zero());
                // SAFETY: we checked its existence already, we assume we have rights to read it
                let options: TestOptions =
                    serde_json::from_str(options_path.get_buffer_from_file().as_str()).unwrap();

                for test_case in options.cases {
                    let format_options: JsFormatOptions = test_case.into();
                    // we don't track the source type inside the serializable structs, so we
                    // inject it here
                    let formatted = format_node(format_options.clone(), &root).unwrap();
                    let printed = formatted.print().unwrap();

                    if !has_errors {
                        check_reformat::check_reformat(check_reformat::CheckReformatParams {
                            root: &root,
                            text: printed.as_code(),
                            source_type,
                            file_name,
                            options: format_options.clone(),
                        });
                    }

                    snapshot_content.add_output(printed, format_options);
                }
            }
        }

        insta::with_settings!({
            prepend_module_to_snapshot => false,
            snapshot_path => spec_input_file.parent().unwrap(),
        }, {
            insta::assert_snapshot!(file_name, snapshot_content.snap_content(), file_name);
        });
    }
}
