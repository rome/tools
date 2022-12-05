use crate::check_reformat::{CheckReformat, CheckReformatParams};
use crate::snapshot_builder::{SnapshotBuilder, SnapshotOutput};
use crate::TestFormatLanguage;
use rome_diagnostics::FileId;
use rome_formatter::{format_node, FormatContext, FormatLanguage, FormatOptions};
use rome_fs::RomePath;
use rome_service::workspace::{FeatureName, SupportsFeatureParams};
use rome_service::App;
use std::path::{Path, PathBuf};

pub struct SpecTestFile<'a> {
    input_file: RomePath,
    root_path: &'a Path,

    input_code: String,
}

impl<'a> SpecTestFile<'a> {
    pub fn new(input_file: &'a str, root_path: &'a Path) -> Option<SpecTestFile<'a>> {
        let app = App::default();
        let file_path = &input_file;
        let spec_input_file = Path::new(input_file);

        assert!(
            spec_input_file.is_file(),
            "The input '{}' must exist and be a file.",
            spec_input_file.display()
        );

        let mut input_file = RomePath::new(file_path, FileId::zero());
        let can_format = app
            .workspace
            .supports_feature(SupportsFeatureParams {
                path: input_file.clone(),
                feature: FeatureName::Format,
            })
            .unwrap();

        match can_format.reason {
            None => {
                let input_code = input_file.get_buffer_from_file();

                Some(SpecTestFile {
                    input_file,
                    root_path,

                    input_code,
                })
            }
            Some(_) => None,
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
}

pub struct SpecSnapshot<'a, L> {
    test_file: SpecTestFile<'a>,
    test_directory: PathBuf,
    language: L,
}

impl<'a, L> SpecSnapshot<'a, L>
where
    L: TestFormatLanguage,
{
    pub fn new(test_file: SpecTestFile<'a>, test_directory: &str, language: L) -> Self {
        let test_directory = PathBuf::from(test_directory);

        SpecSnapshot {
            test_file,
            test_directory,
            language,
        }
    }

    pub fn test(self) where <<<L as TestFormatLanguage>::FormatLanguage as FormatLanguage>::Context as FormatContext>::Options: std::fmt::Display{
        let input_file = self.test_file().input_file().as_path();

        let mut snapshot_builder = SnapshotBuilder::new(input_file)
            .with_input(self.test_file.input_code())
            .with_separator()
            .with_multiple_outputs();

        let parsed = self.language.parse(self.test_file.input_code());

        let has_errors = parsed.has_errors();
        let root = parsed.syntax();

        let formatted = format_node(&root, self.language.format_language()).unwrap();
        let printed = formatted.print().unwrap();

        if !has_errors {
            let check_reformat = CheckReformat::new(
                CheckReformatParams::new(&root, printed.as_code(), self.test_file.file_name()),
                &self.language,
            );
            check_reformat.check_reformat();
        }

        let max_width = self
            .language
            .format_language()
            .options()
            .line_width()
            .value() as usize;

        snapshot_builder = snapshot_builder
            .with_output_and_options(
                SnapshotOutput::new(printed.as_code()).with_index(1),
                self.language.format_language().options(),
            )
            .with_unimplemented(&printed)
            .with_lines_exceeding_max_width(printed.as_code(), max_width);

        let options_path = self.test_directory.join("options.json");
        if options_path.exists() {
            let mut options_path = RomePath::new(&options_path, FileId::zero());

            // SAFETY: we checked its existence already, we assume we have rights to read it
            let test_languages = self
                .language
                .read_format_languages_from_file(&mut options_path);

            for (index, test_language) in test_languages.iter().enumerate() {
                let formatted = format_node(&root, test_language.clone()).unwrap();
                let printed = formatted.print().unwrap();
                let language = L::from_format_language(test_language);

                if !has_errors {
                    let check_reformat = CheckReformat::new(
                        CheckReformatParams::new(
                            &root,
                            printed.as_code(),
                            self.test_file.file_name(),
                        ),
                        &language,
                    );
                    check_reformat.check_reformat();
                }

                let max_width = language.format_language().options().line_width().value() as usize;

                snapshot_builder = snapshot_builder
                    .with_output_and_options(
                        SnapshotOutput::new(printed.as_code()).with_index(index + 2),
                        language.format_language().options(),
                    )
                    .with_unimplemented(&printed)
                    .with_lines_exceeding_max_width(printed.as_code(), max_width);
            }
        }

        snapshot_builder.finish(self.test_file.relative_file_name());
    }

    fn test_file(&self) -> &SpecTestFile {
        &self.test_file
    }
}
