use rome_rowan::{SyntaxNode, TextRange, TextSize};
use std::{ffi::OsStr, fs::read_to_string, ops::Range, path::Path};

use crate::snapshot_builder::SnapshotBuilder;
use crate::utils::{get_prettier_diff, strip_prettier_placeholders, PrettierDiff};
use rome_formatter::{format_node, format_range, FormatLanguage, FormatOptions};
use rome_parser::AnyParse;

const PRETTIER_IGNORE: &str = "prettier-ignore";
const ROME_IGNORE: &str = "rome-ignore format: prettier ignore";

pub struct PrettierTestFile<'a> {
    input_file: &'static Path,
    root_path: &'a Path,

    input_code: String,
    parse_input: String,

    range_start_index: Option<usize>,
    range_end_index: Option<usize>,
}

impl<'a> PrettierTestFile<'a> {
    pub fn new(input: &'static str, root_path: &'a Path) -> Self {
        let input_file = Path::new(input);

        let mut input_code = read_to_string(input_file)
            .unwrap_or_else(|err| panic!("failed to read {:?}: {:?}", input_file, err));

        let (_, range_start_index, range_end_index) = strip_prettier_placeholders(&mut input_code);
        let parse_input = input_code.replace(PRETTIER_IGNORE, ROME_IGNORE);

        PrettierTestFile {
            input_file,
            root_path,

            input_code,
            parse_input,

            range_start_index,
            range_end_index,
        }
    }

    fn range(&self) -> (Option<usize>, Option<usize>) {
        (self.range_start_index, self.range_end_index)
    }

    pub fn input_file(&self) -> &Path {
        self.input_file
    }

    pub fn parse_input(&self) -> &str {
        self.parse_input.as_str()
    }

    pub fn file_name(&self) -> &str {
        self.input_file
            .file_name()
            .and_then(OsStr::to_str)
            .expect("failed to get file name")
    }

    pub fn file_extension(&self) -> &OsStr {
        self.input_file
            .extension()
            .expect("failed to get file extension")
    }

    pub fn relative_file_name(&self) -> &'static str {
        self.input_file
            .strip_prefix(self.root_path)
            .unwrap_or_else(|_| {
                panic!(
                    "failed to strip prefix {:?} from {:?}",
                    self.root_path, self.input_file
                )
            })
            .to_str()
            .expect("failed to get relative file name")
    }
}

pub trait PrettierTestSnapshot<L>
where
    L: FormatLanguage + Clone + 'static,
{
    fn test_file(&self) -> &PrettierTestFile;

    fn format_language(&self) -> L;

    fn parsed(&self) -> &AnyParse;

    fn check_reformat(&self, root: &SyntaxNode<L::SyntaxLanguage>, formatted: &str);

    fn formatted(&self) -> Option<String> {
        let parsed = self.parsed();
        let has_errors = parsed.has_errors();
        let syntax = parsed.syntax();

        let range = self.test_file().range();

        let result = match range {
            (Some(start), Some(end)) => {
                // Skip the reversed range tests as its impossible
                // to create a reversed TextRange anyway
                if end < start {
                    return None;
                }

                format_range(
                    &syntax,
                    TextRange::new(
                        TextSize::try_from(start).unwrap(),
                        TextSize::try_from(end).unwrap(),
                    ),
                    self.format_language(),
                )
            }
            _ => format_node(&syntax, self.format_language())
                .map(|formatted| formatted.print().unwrap()),
        };

        let formatted = result.expect("formatting failed");
        let formatted = match range {
            (Some(_), Some(_)) => {
                let range = formatted
                    .range()
                    .expect("the result of format_range should have a range");

                let formatted = formatted.as_code();
                let mut output_code = self.test_file().parse_input.clone();
                output_code.replace_range(Range::<usize>::from(range), formatted);
                output_code
            }
            _ => {
                let formatted = formatted.into_code();

                if !has_errors {
                    self.check_reformat(&syntax, &formatted);
                }

                formatted
            }
        };

        let formatted = formatted.replace(ROME_IGNORE, PRETTIER_IGNORE);

        Some(formatted)
    }

    fn test(self)
    where
        Self: Sized,
    {
        let formatted = match self.formatted() {
            Some(formatted) => formatted,
            None => return,
        };

        let relative_file_name = self.test_file().relative_file_name();
        let input_file = self.test_file().input_file;

        let prettier_diff = get_prettier_diff(input_file, relative_file_name, &formatted);

        let prettier_diff = match prettier_diff {
            PrettierDiff::Diff(prettier_diff) => prettier_diff,
            PrettierDiff::Same => return,
        };

        let mut builder = SnapshotBuilder::new(input_file)
            .with_input(&self.test_file().input_code)
            .with_prettier_diff(&prettier_diff)
            .with_output(&formatted)
            .with_errors(self.parsed(), &self.test_file().parse_input);

        let max_width = self.format_language().options().line_width().value() as usize;
        builder = builder.with_lines_exceeding_max_width(&formatted, max_width);

        builder.finish(relative_file_name);
    }
}
