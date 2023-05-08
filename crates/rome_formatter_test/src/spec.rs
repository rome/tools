use crate::check_reformat::CheckReformat;
use crate::snapshot_builder::{SnapshotBuilder, SnapshotOutput};
use crate::utils::strip_rome_placeholders;
use crate::TestFormatLanguage;
use rome_console::EnvConsole;
use rome_formatter::{FormatOptions, Printed};
use rome_fs::RomePath;
use rome_parser::AnyParse;
use rome_rowan::{TextRange, TextSize};
use rome_service::workspace::{FeatureName, FeaturesBuilder, SupportsFeatureParams};
use rome_service::App;
use std::ops::Range;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SpecTestFile<'a> {
    input_file: RomePath,
    root_path: &'a Path,

    input_code: String,

    range_start_index: Option<usize>,
    range_end_index: Option<usize>,
}

impl<'a> SpecTestFile<'a> {
    pub fn try_from_file(input_file: &'a str, root_path: &'a Path) -> Option<SpecTestFile<'a>> {
        let mut console = EnvConsole::default();
        let app = App::with_console(&mut console);
        let file_path = &input_file;
        let spec_input_file = Path::new(input_file);

        assert!(
            spec_input_file.is_file(),
            "The input '{}' must exist and be a file.",
            spec_input_file.display()
        );

        let mut input_file = RomePath::new(file_path);
        let can_format = app
            .workspace
            .file_features(SupportsFeatureParams {
                path: input_file.clone(),
                feature: FeaturesBuilder::new().with_formatter().build(),
            })
            .unwrap();

        if can_format.supports_for(&FeatureName::Format) {
            let mut input_code = input_file.get_buffer_from_file();

            let (_, range_start_index, range_end_index) = strip_rome_placeholders(&mut input_code);

            Some(SpecTestFile {
                input_file,
                root_path,

                input_code,

                range_start_index,
                range_end_index,
            })
        } else {
            None
        }
    }

    pub fn input_code(&self) -> &str {
        &self.input_code
    }

    pub fn file_name(&self) -> &str {
        self.input_file.file_name().unwrap().to_str().unwrap()
    }

    pub fn input_file(&self) -> &RomePath {
        &self.input_file
    }

    pub fn relative_file_name(&self) -> &str {
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

    fn range(&self) -> (Option<usize>, Option<usize>) {
        (self.range_start_index, self.range_end_index)
    }
}

pub struct SpecSnapshot<'a, L>
where
    L: TestFormatLanguage,
{
    test_file: SpecTestFile<'a>,
    test_directory: PathBuf,
    language: L,
    options: L::Options,
}

impl<'a, L> SpecSnapshot<'a, L>
where
    L: TestFormatLanguage,
{
    pub fn new(
        test_file: SpecTestFile<'a>,
        test_directory: &str,
        language: L,
        options: L::Options,
    ) -> Self {
        let test_directory = PathBuf::from(test_directory);

        SpecSnapshot {
            test_file,
            test_directory,
            language,
            options,
        }
    }

    fn formatted(&self, parsed: &AnyParse, options: L::Options) -> (String, Printed) {
        let has_errors = parsed.has_errors();
        let syntax = parsed.syntax();

        let range = self.test_file.range();

        let result = match range {
            (Some(start), Some(end)) => self.language.format_range(
                options.clone(),
                &syntax,
                TextRange::new(
                    TextSize::try_from(start).unwrap(),
                    TextSize::try_from(end).unwrap(),
                ),
            ),
            _ => self
                .language
                .format_node(options.clone(), &syntax)
                .map(|formatted| formatted.print().unwrap()),
        };
        let formatted = result.expect("formatting failed");

        let output_code = match range {
            (Some(_), Some(_)) => {
                let range = formatted
                    .range()
                    .expect("the result of format_range should have a range");

                let mut output_code = self.test_file.input_code.clone();
                output_code.replace_range(Range::<usize>::from(range), formatted.as_code());

                // Check if output code is a valid syntax
                let parsed = self.language.parse(&output_code);

                if parsed.has_errors() {
                    panic!(
                        "{:?} format range produced an invalid syntax tree: {:?}",
                        self.test_file.input_file, output_code
                    )
                }

                output_code
            }
            _ => {
                let output_code = formatted.as_code();

                if !has_errors {
                    let check_reformat = CheckReformat::new(
                        &syntax,
                        output_code,
                        self.test_file.file_name(),
                        &self.language,
                        options,
                    );
                    check_reformat.check_reformat();
                }

                output_code.to_string()
            }
        };

        (output_code, formatted)
    }

    pub fn test(self) {
        let input_file = self.test_file().input_file().as_path();

        let mut snapshot_builder = SnapshotBuilder::new(input_file)
            .with_input(self.test_file.input_code())
            .with_separator()
            .with_multiple_outputs();

        let parsed = self.language.parse(self.test_file.input_code());

        let (output_code, printed) = self.formatted(&parsed, self.options.clone());

        let max_width = self.options.line_width().value() as usize;

        snapshot_builder = snapshot_builder
            .with_output_and_options(
                SnapshotOutput::new(&output_code).with_index(1),
                self.options.clone(),
            )
            .with_unimplemented(&printed)
            .with_lines_exceeding_max_width(&output_code, max_width);

        let options_path = self.test_directory.join("options.json");
        if options_path.exists() {
            let mut options_path = RomePath::new(&options_path);

            // SAFETY: we checked its existence already, we assume we have rights to read it
            let test_options = self
                .language
                .deserialize_format_options(options_path.get_buffer_from_file().as_str());

            for (index, options) in test_options.into_iter().enumerate() {
                let (output_code, printed) = self.formatted(&parsed, options.clone());

                let max_width = options.line_width().value() as usize;

                snapshot_builder = snapshot_builder
                    .with_output_and_options(
                        SnapshotOutput::new(&output_code).with_index(index + 2),
                        options,
                    )
                    .with_unimplemented(&printed)
                    .with_lines_exceeding_max_width(&output_code, max_width);
            }
        }

        snapshot_builder.finish(self.test_file.relative_file_name());
    }

    fn test_file(&self) -> &SpecTestFile {
        &self.test_file
    }
}
