use crate::TestFormatLanguage;
use rome_diagnostics::console::fmt::{Formatter, Termcolor};
use rome_diagnostics::console::markup;
use rome_diagnostics::termcolor;
use rome_diagnostics::{DiagnosticExt, PrintDiagnostic};
use rome_formatter::FormatLanguage;
use rome_rowan::SyntaxNode;

pub struct CheckReformatParams<'a, L>
where
    L: FormatLanguage + 'static,
{
    root: &'a SyntaxNode<L::SyntaxLanguage>,
    text: &'a str,
    file_name: &'a str,
}

impl<'a, L> CheckReformatParams<'a, L>
where
    L: FormatLanguage + 'static,
{
    pub fn new(root: &'a SyntaxNode<L::SyntaxLanguage>, text: &'a str, file_name: &'a str) -> Self {
        CheckReformatParams {
            root,
            text,
            file_name,
        }
    }
}

/// Perform a second pass of formatting on a file, printing a diff if the
/// output doesn't match the input
///
pub struct CheckReformat<'a, 'b, L>
where
    L: TestFormatLanguage,
{
    params: CheckReformatParams<'a, L::FormatLanguage>,
    language: &'b L,
    options: L::Options,
}

impl<'a, 'b, L> CheckReformat<'a, 'b, L>
where
    L: TestFormatLanguage,
{
    pub fn new(
        params: CheckReformatParams<'a, L::FormatLanguage>,
        language: &'b L,
        options: L::Options,
    ) -> Self {
        CheckReformat {
            params,
            language,
            options,
        }
    }

    pub fn check_reformat(&self) {
        let CheckReformatParams {
            root,
            text,
            file_name,
        } = self.params;

        let re_parse = self.language.parse(text);

        // Panic if the result from the formatter has syntax errors
        if re_parse.has_errors() {
            let mut buffer = termcolor::Buffer::ansi();

            for diagnostic in re_parse.diagnostics() {
                let error = diagnostic
                    .clone()
                    .with_file_path(file_name)
                    .with_file_source_code(text.to_string());
                Formatter::new(&mut Termcolor(&mut buffer))
                    .write_markup(markup! {
                        {PrintDiagnostic::verbose(&error)}
                    })
                    .expect("failed to emit diagnostic");
            }

            panic!(
                "formatter output had syntax errors where input had none:\n{}",
                std::str::from_utf8(buffer.as_slice()).expect("non utf8 in error buffer")
            )
        }

        let formatted = self
            .language
            .format_node(self.options.clone(), &re_parse.syntax())
            .unwrap();
        let printed = formatted.print().unwrap();

        if text != printed.as_code() {
            let input_format_element = self
                .language
                .format_node(self.options.clone(), root)
                .unwrap();
            let pretty_input_ir = format!("{}", formatted.into_document());
            let pretty_reformat_ir = format!("{}", input_format_element.into_document());

            // Print a diff of the Formatter IR emitted for the input and the output
            let diff = similar_asserts::SimpleDiff::from_str(
                &pretty_input_ir,
                &pretty_reformat_ir,
                "input",
                "output",
            );

            println!("{diff}");

            similar_asserts::assert_eq!(text, printed.as_code());
        }
    }
}
