use rome_diagnostics::console::fmt::{Formatter, Termcolor};
use rome_diagnostics::console::markup;
use rome_diagnostics::termcolor;
use rome_diagnostics::{DiagnosticExt, PrintDiagnostic};
use rome_formatter::{format_node, FormatLanguage};
use rome_parser::AnyParse;
use rome_rowan::SyntaxNode;

pub struct CheckReformatParams<'a, L>
where
    L: FormatLanguage + Clone + 'static,
{
    pub root: &'a SyntaxNode<L::SyntaxLanguage>,
    pub format_language: L,
    pub text: &'a str,
    pub file_name: &'a str,
}

/// Perform a second pass of formatting on a file, printing a diff if the
/// output doesn't match the input
pub trait CheckReformat<L>
where
    L: FormatLanguage + Clone + 'static,
{
    fn parse(&self, text: &str) -> AnyParse;

    fn params(&self) -> CheckReformatParams<L>;

    fn check_reformat(&self) {
        let CheckReformatParams {
            root,
            format_language,
            text,
            file_name,
        } = self.params();

        let re_parse = self.parse(text);

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

        let formatted = format_node(&re_parse.syntax(), format_language.clone()).unwrap();
        let printed = formatted.print().unwrap();

        if text != printed.as_code() {
            let input_format_element = format_node(root, format_language).unwrap();
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
