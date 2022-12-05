use rome_diagnostics::console::fmt::{Formatter, Termcolor};
use rome_diagnostics::console::markup;
use rome_diagnostics::PrintDiagnostic;
use rome_diagnostics::{termcolor, DiagnosticExt};
use rome_parser::AnyParse;
use std::ffi::OsStr;
use std::fmt::Write;
use std::path::Path;

#[derive(serde::Serialize)]
struct TestInfo {
    test_file: String,
}

pub struct SnapshotBuilder<'a> {
    input_file: &'a Path,
    snapshot: String,
}

impl<'a> SnapshotBuilder<'a> {
    pub fn new(input_file: &'a Path) -> Self {
        SnapshotBuilder {
            input_file,
            snapshot: String::new(),
        }
    }

    pub fn with_input(mut self, input: &str) -> Self {
        writeln!(self.snapshot).unwrap();
        writeln!(self.snapshot, "# Input").unwrap();
        writeln!(self.snapshot).unwrap();
        self.write_extension();
        self.snapshot.push_str(input);
        writeln!(self.snapshot, "```").unwrap();
        writeln!(self.snapshot).unwrap();
        writeln!(self.snapshot).unwrap();

        self
    }

    pub fn with_prettier_diff(mut self, prettier_diff: &str) -> Self {
        writeln!(self.snapshot, "# Prettier differences").unwrap();
        writeln!(self.snapshot).unwrap();
        writeln!(self.snapshot, "```diff").unwrap();
        self.snapshot.push_str(prettier_diff);
        writeln!(self.snapshot, "```").unwrap();
        writeln!(self.snapshot).unwrap();

        self
    }

    pub fn with_output(mut self, output: &str) -> Self {
        writeln!(self.snapshot, "# Output").unwrap();
        writeln!(self.snapshot).unwrap();
        self.write_extension();
        self.snapshot.push_str(output);
        writeln!(self.snapshot, "```").unwrap();
        writeln!(self.snapshot).unwrap();
        writeln!(self.snapshot).unwrap();

        self
    }

    pub fn with_errors(mut self, parsed: &AnyParse, parse_input: &str) -> Self {
        if !parsed.has_errors() {
            return self;
        }

        let file_name = self.input_file.file_name().and_then(OsStr::to_str).unwrap();

        let mut buffer = termcolor::Buffer::no_color();

        for diagnostic in parsed.diagnostics() {
            let error = diagnostic
                .clone()
                .with_file_path(file_name)
                .with_file_source_code(parse_input);
            Formatter::new(&mut Termcolor(&mut buffer))
                .write_markup(markup! {
                    {PrintDiagnostic::verbose(&error)}
                })
                .expect("failed to emit diagnostic");
        }

        writeln!(self.snapshot, "# Errors").unwrap();
        writeln!(self.snapshot, "```").unwrap();
        writeln!(
            self.snapshot,
            "{}",
            std::str::from_utf8(buffer.as_slice()).expect("non utf8 in error buffer")
        )
        .unwrap();
        writeln!(self.snapshot, "```").unwrap();
        writeln!(self.snapshot).unwrap();

        self
    }

    pub fn with_lines_exceeding_max_width(mut self, output: &str, max_width: usize) -> Self {
        let mut lines_exceeding_max_width = output
            .lines()
            .enumerate()
            .filter(|(_, line)| line.len() > max_width)
            .peekable();

        if lines_exceeding_max_width.peek().is_some() {
            writeln!(
                self.snapshot,
                "# Lines exceeding max width of {max_width} characters"
            )
            .unwrap();
            writeln!(self.snapshot, "```").unwrap();

            for (index, line) in lines_exceeding_max_width {
                let line_number = index + 1;
                writeln!(self.snapshot, "{line_number:>5}: {line}").unwrap();
            }
            writeln!(self.snapshot, "```").unwrap();
        }

        self
    }

    pub fn finish(self, relative_file_name: &str) {
        let file_name = self.input_file.file_name().and_then(OsStr::to_str).unwrap();

        let info = TestInfo {
            test_file: relative_file_name.to_owned(),
        };

        insta::with_settings!({
            prepend_module_to_snapshot => false,
            snapshot_path => self.input_file.parent().unwrap(),
            omit_expression => true,
            info => &info
        }, {
            insta::assert_snapshot!(file_name, self.snapshot);
        });
    }
}

impl SnapshotBuilder<'_> {
    fn write_extension(&mut self) {
        let file_extension = self.input_file.extension().unwrap().to_str().unwrap();
        writeln!(self.snapshot, "```{file_extension}").unwrap();
    }
}
