use rome_diagnostics::console::fmt::{Formatter, Termcolor};
use rome_diagnostics::console::markup;
use rome_diagnostics::{location::FileId, termcolor};
use rome_diagnostics::{DiagnosticExt, PrintDiagnostic};
use rome_js_formatter::context::JsFormatOptions;
use rome_js_formatter::format_node;
use rome_js_parser::parse;
use rome_js_syntax::{JsSyntaxNode, SourceType};

pub struct CheckReformatParams<'a> {
    pub root: &'a JsSyntaxNode,
    pub text: &'a str,
    pub source_type: SourceType,
    pub file_name: &'a str,
    pub options: JsFormatOptions,
}

/// Perform a second pass of formatting on a file, printing a diff if the
/// output doesn't match the input
pub fn check_reformat(params: CheckReformatParams) {
    let CheckReformatParams {
        root,
        text,
        source_type,
        file_name,
        options,
    } = params;

    let re_parse = parse(text, FileId::zero(), source_type);

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
                    {PrintDiagnostic(&error, true)}
                })
                .expect("failed to emit diagnostic");
        }

        panic!(
            "formatter output had syntax errors where input had none:\n{}",
            std::str::from_utf8(buffer.as_slice()).expect("non utf8 in error buffer")
        )
    }

    let formatted = format_node(options.clone(), &re_parse.syntax()).unwrap();
    let printed = formatted.print().unwrap();

    if text != printed.as_code() {
        let input_format_element = format_node(options, root).unwrap();
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
