//! Extremely fast, lossless, and error tolerant JSON Parser.

pub use crate::lexer::{Lexer, Token};
use rome_console::fmt::Display;
use rome_console::{markup, MarkupBuf};
use rome_diagnostics::location::AsSpan;
use rome_diagnostics::{
    Advices, Diagnostic, FileId, Location, LogCategory, MessageAndDescription, Visit,
};
use rome_json_syntax::TextRange;

mod lexer;

#[derive(Debug, Diagnostic, Clone)]
#[diagnostic(category = "parse", severity = Error)]
pub struct ParseDiagnostic {
    /// The location where the error is occurred
    #[location(span)]
    span: Option<TextRange>,
    /// Reference to a file where the issue occurred
    #[location(resource)]
    file_id: FileId,
    #[message]
    #[description]
    message: MessageAndDescription,
    #[advice]
    advice: ParserAdvice,
}

/// Possible details related to the diagnostic
#[derive(Debug, Default, Clone)]
struct ParserAdvice {
    /// A list a possible details that can be attached to the diagnostic.
    /// Useful to explain the nature errors.
    detail_list: Vec<ParserAdviceDetail>,
    /// A message for the user that should tell the user how to fix the issue
    hint: Option<MarkupBuf>,
}

/// The structure of the advice. A message that gives details, a possible range so
/// the diagnostic is able to highlight the part of the code we want to explain.
#[derive(Debug, Clone)]
struct ParserAdviceDetail {
    /// A message that should explain this detail
    message: MarkupBuf,
    /// An optional range that should highlight the details of the code
    span: Option<TextRange>,
    /// The file id, reference to the actual file
    file_id: FileId,
}

impl ParserAdvice {
    fn add_detail(&mut self, message: impl Display, range: Option<TextRange>, file_id: FileId) {
        self.detail_list.push(ParserAdviceDetail {
            message: markup! { {message} }.to_owned(),
            span: range,
            file_id,
        });
    }

    fn add_hint(&mut self, message: impl Display) {
        self.hint = Some(markup! { { message } }.to_owned());
    }
}

impl Advices for ParserAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        for detail in &self.detail_list {
            let ParserAdviceDetail {
                span,
                message,
                file_id,
            } = detail;
            visitor.record_log(LogCategory::Info, &markup! { {message} }.to_owned())?;
            let location = Location::builder().span(span).resource(file_id).build();
            visitor.record_frame(location)?;
        }
        if let Some(hint) = &self.hint {
            visitor.record_log(LogCategory::Info, &markup! { {hint} }.to_owned())?;
        }
        Ok(())
    }
}

impl ParseDiagnostic {
    pub fn new(file_id: FileId, message: impl Display, span: impl AsSpan) -> Self {
        Self {
            file_id,
            span: span.as_span(),
            message: MessageAndDescription::from(markup! { {message} }.to_owned()),
            advice: ParserAdvice::default(),
        }
    }

    pub const fn is_error(&self) -> bool {
        true
    }

    /// Use this API if you want to highlight more code frame, to help to explain where's the error.
    ///
    /// A detail is printed **after the actual error** and before the hint.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use rome_console::fmt::{Termcolor};
    /// # use rome_console::markup;
    /// # use rome_diagnostics::{DiagnosticExt, FileId, PrintDiagnostic, console::fmt::Formatter};
    /// # use rome_json_parser::ParseDiagnostic;
    /// # use rome_json_syntax::TextRange;
    /// # use rome_rowan::TextSize;
    /// # use std::fmt::Write;
    ///
    /// let source = "const a";
    /// let range = TextRange::new(TextSize::from(0), TextSize::from(5));
    /// let mut diagnostic = ParseDiagnostic::new(FileId::zero(), "this is wrong!", range)
    ///     .detail(TextRange::new(TextSize::from(6), TextSize::from(7)), "This is reason why it's broken");
    ///
    /// let mut write = rome_diagnostics::termcolor::Buffer::no_color();
    /// let error = diagnostic
    ///     .clone()
    ///     .with_file_path(FileId::zero())
    ///     .with_file_source_code(source.to_string());
    /// Formatter::new(&mut Termcolor(&mut write))
    ///     .write_markup(markup! {
    ///     {PrintDiagnostic(&error)}
    /// })
    ///     .expect("failed to emit diagnostic");
    ///
    /// let mut result = String::new();
    /// write!(
    ///     result,
    ///     "{}",
    ///     std::str::from_utf8(write.as_slice()).expect("non utf8 in error buffer")
    /// ).expect("");
    ///
    /// let expected = r#"parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    ///
    ///   × this is wrong!
    ///  
    ///   > 1 │ const a
    ///       │ ^^^^^
    ///  
    ///   i This is reason why it's broken
    ///  
    ///   > 1 │ const a
    ///       │       ^
    ///  
    /// "#;
    /// assert_eq!(result, expected);
    pub fn detail(mut self, range: impl AsSpan, message: impl Display) -> Self {
        self.advice
            .add_detail(message, range.as_span(), self.file_id);
        self
    }

    /// Small message that should suggest the user how they could fix the error
    ///
    /// Hints are rendered a **last part** of the diagnostics
    ///
    /// ## Examples
    ///
    /// ```
    /// # use rome_console::fmt::{Termcolor};
    /// # use rome_console::markup;
    /// # use rome_diagnostics::{DiagnosticExt, FileId, PrintDiagnostic, console::fmt::Formatter};
    /// # use rome_json_parser::ParseDiagnostic;
    /// # use rome_json_syntax::TextRange;
    /// # use rome_rowan::TextSize;
    /// # use std::fmt::Write;
    ///
    /// let source = "const a";
    /// let range = TextRange::new(TextSize::from(0), TextSize::from(5));
    /// let mut diagnostic = ParseDiagnostic::new(FileId::zero(), "this is wrong!", range)
    ///     .hint("You should delete the code");
    ///
    /// let mut write = rome_diagnostics::termcolor::Buffer::no_color();
    /// let error = diagnostic
    ///     .clone()
    ///     .with_file_path(FileId::zero())
    ///     .with_file_source_code(source.to_string());
    /// Formatter::new(&mut Termcolor(&mut write))
    ///     .write_markup(markup! {
    ///     {PrintDiagnostic(&error)}
    /// })
    ///     .expect("failed to emit diagnostic");
    ///
    /// let mut result = String::new();
    /// write!(
    ///     result,
    ///     "{}",
    ///     std::str::from_utf8(write.as_slice()).expect("non utf8 in error buffer")
    /// ).expect("");
    ///
    /// let expected = r#"parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    ///
    ///   × this is wrong!
    ///  
    ///   > 1 │ const a
    ///       │ ^^^^^
    ///  
    ///   i You should delete the code
    ///  
    /// "#;
    /// assert_eq!(result, expected);
    /// ```
    ///
    pub fn hint(mut self, message: impl Display) -> Self {
        self.advice.add_hint(message);
        self
    }
}
