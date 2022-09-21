use std::{borrow::Cow, io, iter};

use rome_console::{
    codespan::SourceFile,
    diff::{Diff, DiffMode},
    fmt, markup, Markup, MarkupBuf, MarkupElement, MarkupNode,
};
use unicode_width::UnicodeWidthStr;

mod backtrace;
mod frame;

use super::{
    diagnostic::internal::AsDiagnostic, Advices, Diagnostic, DiagnosticTags, Location, LogCategory,
    Resource, Severity, Visit,
};

pub use self::backtrace::{set_bottom_frame, Backtrace};

/// Helper struct from printing the description of a diagnostic into any
/// formatter implementing [std::fmt::Write].
pub struct PrintDescription<'fmt, D: ?Sized>(pub &'fmt D);

impl<'fmt, D: AsDiagnostic + ?Sized> std::fmt::Display for PrintDescription<'fmt, D> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .as_diagnostic()
            .description(fmt)
            .map_err(|_| std::fmt::Error)
    }
}

/// Helper struct for printing a diagnostic as markup into any formatter
/// implementing [rome_console::fmt::Write].
pub struct PrintDiagnostic<'fmt, D: ?Sized>(pub &'fmt D);

impl<'fmt, D: AsDiagnostic + ?Sized> fmt::Display for PrintDiagnostic<'fmt, D> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let diagnostic = self.0.as_diagnostic();

        // Print the header for the diagnostic
        fmt.write_markup(markup! {
            {PrintHeader(diagnostic)}"\n\n"
        })?;

        // Wrap the formatter with an indentation level and print the advices
        let mut slot = None;
        let mut fmt = IndentWriter::wrap(fmt, &mut slot, true, "  ");
        let mut visitor = PrintAdvices(&mut fmt);

        print_advices(&mut visitor, diagnostic, true)
    }
}

/// Display struct implementing the formatting of a diagnostic header.
struct PrintHeader<'fmt, D: ?Sized>(&'fmt D);

impl<'fmt, D: Diagnostic + ?Sized> fmt::Display for PrintHeader<'fmt, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let Self(diagnostic) = *self;

        // Wrap the formatter with a counter to measure the width of the printed text
        let mut slot = None;
        let mut fmt = CountWidth::wrap(f, &mut slot);

        // Print the diagnostic location if it has one
        if let Some(location) = diagnostic.location() {
            // Print the path if it's a file
            let file_name = match &location.resource {
                Resource::File(file) => file.path(),
                _ => None,
            };

            if let Some(name) = file_name {
                fmt.write_str(name)?;

                // Print the line and column position if the location has a span and source code
                // (the source code is necessary to convert a byte offset into a line + column)
                if let (Some(span), Some(source_code)) = (location.span, location.source_code) {
                    let line_starts = source_code.line_starts.map_or_else(
                        || Cow::Owned(SourceFile::line_starts(source_code.text).collect()),
                        Cow::Borrowed,
                    );

                    let file = SourceFile::new(source_code.text, line_starts.as_ref());
                    if let Ok(location) = file.location(span.start()) {
                        fmt.write_markup(markup! {
                            ":"{location.line_number}":"{location.column_number}
                        })?;
                    }
                }

                fmt.write_str(" ")?;
            }
        }

        // Print the category of the diagnostic, with a hyperlink if
        // the category has an associated link
        if let Some(category) = diagnostic.category() {
            if let Some(link) = category.link() {
                fmt.write_markup(markup! {
                    <Hyperlink href={link}>{category.name()}</Hyperlink>" "
                })?;
            } else {
                fmt.write_markup(markup! {
                    {category.name()}" "
                })?;
            }
        }

        // Print the internal, fixable and fatal tags
        let tags = diagnostic.tags();

        if tags.contains(DiagnosticTags::INTERNAL) {
            fmt.write_markup(markup! {
                <Inverse><Error>" INTERNAL "</Error></Inverse>" "
            })?;
        }

        if tags.contains(DiagnosticTags::FIXABLE) {
            fmt.write_markup(markup! {
                <Inverse>" FIXABLE "</Inverse>" "
            })?;
        }

        if tags.contains(DiagnosticTags::FATAL) {
            fmt.write_markup(markup! {
                <Inverse><Error>" FATAL "</Error></Inverse>" "
            })?;
        }

        // Load the printed width for the header, and fill the rest of the line
        // with the '━' line character up to 100 characters
        const HEADER_WIDTH: usize = 100;
        let text_width = slot.map_or(0, |writer| writer.width);
        let line_width = HEADER_WIDTH.saturating_sub(text_width);
        f.write_str(&"\u{2501}".repeat(line_width))
    }
}

/// Wrapper for a type implementing [fmt::Write] that counts the total width of
/// all printed characters.
struct CountWidth<'a, W: ?Sized> {
    writer: &'a mut W,
    width: usize,
}

impl<'write> CountWidth<'write, dyn fmt::Write + 'write> {
    /// Wrap the writer in an existing [fmt::Formatter] with an instance of [CountWidth].
    fn wrap<'slot, 'fmt: 'write + 'slot>(
        fmt: &'fmt mut fmt::Formatter<'_>,
        slot: &'slot mut Option<Self>,
    ) -> fmt::Formatter<'slot> {
        fmt.wrap_writer(|writer| slot.get_or_insert(Self { writer, width: 0 }))
    }
}

impl<W: fmt::Write + ?Sized> fmt::Write for CountWidth<'_, W> {
    fn write_str(&mut self, elements: &fmt::MarkupElements<'_>, content: &str) -> io::Result<()> {
        self.writer.write_str(elements, content)?;
        self.width += UnicodeWidthStr::width(content);
        Ok(())
    }

    fn write_fmt(
        &mut self,
        elements: &fmt::MarkupElements<'_>,
        content: std::fmt::Arguments<'_>,
    ) -> io::Result<()> {
        if let Some(content) = content.as_str() {
            self.write_str(elements, content)
        } else {
            let content = content.to_string();
            self.write_str(elements, &content)
        }
    }
}

/// Write the advices for `diagnostic` into `visitor`.
fn print_advices<V, D>(visitor: &mut V, diagnostic: &D, verbose: bool) -> io::Result<()>
where
    V: Visit,
    D: Diagnostic + ?Sized,
{
    // Visit the advices of the diagnostic with a lightweight visitor that
    // detects if the diagnostic has any frame or backtrace advice
    let skip_frame = if let Some(location) = diagnostic.location() {
        let mut frame_visitor = FrameVisitor {
            location,
            skip_frame: false,
        };

        diagnostic.advices(&mut frame_visitor)?;

        frame_visitor.skip_frame
    } else {
        false
    };

    // Print the message for the diagnostic as a log advice
    print_message_advice(visitor, diagnostic, skip_frame)?;

    // Print the other advices for the diagnostic
    diagnostic.advices(visitor)?;

    // Print the tags of the diagnostic as advices
    print_tags_advices(visitor, diagnostic)?;

    // If verbose printing is enabled, print the verbose advices in a nested group
    if verbose {
        // Count the number of verbose advices in the diagnostic
        let mut counter = CountAdvices(0);
        diagnostic.verbose_advices(&mut counter)?;

        // If the diagnostic has any verbose advice, print the group
        if !counter.is_empty() {
            let verbose_advices = PrintVerboseAdvices(diagnostic);
            visitor.record_group(&"Verbose advice", &verbose_advices)?;
        }
    }

    Ok(())
}

/// Advice visitor used to detect if the diagnostic contains any frame or backtrace diagnostic.
#[derive(Debug)]
struct FrameVisitor<'diag> {
    location: Location<'diag>,
    skip_frame: bool,
}

impl Visit for FrameVisitor<'_> {
    fn record_frame(&mut self, location: Location<'_>) -> io::Result<()> {
        if location == self.location {
            self.skip_frame = true;
        }
        Ok(())
    }

    fn record_backtrace(&mut self, _: &dyn fmt::Display, _: &Backtrace) -> io::Result<()> {
        self.skip_frame = true;
        Ok(())
    }
}

/// Print the message and code frame for the diagnostic as advices.
fn print_message_advice<V, D>(visitor: &mut V, diagnostic: &D, skip_frame: bool) -> io::Result<()>
where
    V: Visit,
    D: Diagnostic + ?Sized,
{
    // Print the entire message / cause chain for the diagnostic to a MarkupBuf
    let message = {
        let mut message = MarkupBuf::default();
        let mut fmt = fmt::Formatter::new(&mut message);
        fmt.write_markup(markup!({ PrintCauseChain(diagnostic) }))?;
        message
    };

    // Print a log advice for the message, with a special fallback if the buffer is empty
    if message.is_empty() {
        visitor.record_log(
            LogCategory::None,
            &markup! {
                <Dim>"no diagnostic message provided"</Dim>
            },
        )?;
    } else {
        let category = match diagnostic.severity() {
            Severity::Error => LogCategory::Error,
            Severity::Warning => LogCategory::Warn,
            Severity::Information | Severity::Hint => LogCategory::Info,
        };

        visitor.record_log(category, &message)?;
    }

    // If the diagnostic has no explicit code frame or backtrace advice, print
    // a code frame advice with the location of the diagnostic
    if !skip_frame {
        if let Some(location) = diagnostic.location().filter(|loc| loc.span.is_some()) {
            visitor.record_frame(location)?;
        }
    }

    Ok(())
}

/// Display wrapper for printing the "cause chain" of a diagnostic, with the
/// message of this diagnostic and all of its sources.
struct PrintCauseChain<'fmt, D: ?Sized>(&'fmt D);

impl<'fmt, D: Diagnostic + ?Sized> fmt::Display for PrintCauseChain<'fmt, D> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let Self(diagnostic) = *self;

        diagnostic.message(fmt)?;

        let chain = iter::successors(diagnostic.source(), |prev| prev.source());
        for diagnostic in chain {
            fmt.write_str("\n\nCaused by:\n")?;

            let mut slot = None;
            let mut fmt = IndentWriter::wrap(fmt, &mut slot, true, "  ");
            diagnostic.message(&mut fmt)?;
        }

        Ok(())
    }
}

/// Implementation of [Visitor] that prints the advices for a diagnostic.
struct PrintAdvices<'a, 'b>(&'a mut fmt::Formatter<'b>);

impl PrintAdvices<'_, '_> {
    fn print_log(
        &mut self,
        kind: MarkupElement<'_>,
        prefix: char,
        text: &dyn fmt::Display,
    ) -> io::Result<()> {
        self.0.write_markup(Markup(&[MarkupNode {
            elements: &[MarkupElement::Emphasis, kind.clone()],
            content: &prefix as &dyn fmt::Display,
        }]))?;

        self.0.write_str(" ")?;

        let mut slot = None;
        let mut fmt = IndentWriter::wrap(self.0, &mut slot, false, "  ");
        fmt.write_markup(Markup(&[MarkupNode {
            elements: &[kind],
            content: text,
        }]))?;

        self.0.write_str("\n\n")
    }
}

impl Visit for PrintAdvices<'_, '_> {
    fn record_log(&mut self, category: LogCategory, text: &dyn fmt::Display) -> io::Result<()> {
        match category {
            LogCategory::None => self.0.write_markup(markup! { {text}"\n\n" }),
            LogCategory::Info => self.print_log(MarkupElement::Info, '\u{2139}', text),
            LogCategory::Warn => self.print_log(MarkupElement::Warn, '\u{26a0}', text),
            LogCategory::Error => self.print_log(MarkupElement::Error, '\u{2716}', text),
        }
    }

    fn record_list(&mut self, list: &[&dyn fmt::Display]) -> io::Result<()> {
        for item in list {
            let mut slot = None;
            let mut fmt = IndentWriter::wrap(self.0, &mut slot, false, "  ");
            fmt.write_markup(markup! {
                "- "{*item}"\n"
            })?;
        }

        if list.is_empty() {
            Ok(())
        } else {
            self.0.write_str("\n")
        }
    }

    fn record_frame(&mut self, location: Location<'_>) -> io::Result<()> {
        frame::print_frame(self.0, location)
    }

    fn record_diff(&mut self, left: &str, right: &str) -> io::Result<()> {
        self.0.write_markup(markup! {
            {Diff { mode: DiffMode::Unified, left, right }}"\n"
        })
    }

    fn record_backtrace(
        &mut self,
        title: &dyn fmt::Display,
        backtrace: &Backtrace,
    ) -> io::Result<()> {
        let mut backtrace = backtrace.clone();
        backtrace.resolve();

        if backtrace.is_empty() {
            return Ok(());
        }

        self.record_log(LogCategory::Info, title)?;

        backtrace::print_backtrace(self.0, &backtrace)
    }

    fn record_command(&mut self, command: &str) -> io::Result<()> {
        self.0.write_markup(markup! {
            <Emphasis>"$"</Emphasis>" "{command}"\n\n"
        })
    }

    fn record_group(&mut self, title: &dyn fmt::Display, advice: &dyn Advices) -> io::Result<()> {
        self.0.write_markup(markup! {
            <Emphasis>{title}</Emphasis>"\n\n"
        })?;

        let mut slot = None;
        let mut fmt = IndentWriter::wrap(self.0, &mut slot, true, "  ");
        let mut visitor = PrintAdvices(&mut fmt);
        advice.record(&mut visitor)
    }
}

/// Print the fatal and internal tags for the diagnostic as log advices.
fn print_tags_advices<V, D>(visitor: &mut V, diagnostic: &D) -> io::Result<()>
where
    V: Visit,
    D: Diagnostic + ?Sized,
{
    let tags = diagnostic.tags();

    if tags.contains(DiagnosticTags::FATAL) {
        visitor.record_log(LogCategory::Warn, &"Rome exited as this error could not be handled and resulted in a fatal error. Please report it if necessary.")?;
    }

    if tags.contains(DiagnosticTags::INTERNAL) {
        visitor.record_log(LogCategory::Warn, &"This diagnostic was derived from an internal Rome error. Potential bug, please report it if necessary.")?;
    }

    Ok(())
}

/// Advice visitor that counts how many advices are visited.
struct CountAdvices(usize);

impl CountAdvices {
    fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl Visit for CountAdvices {
    fn record_log(&mut self, _: LogCategory, _: &dyn fmt::Display) -> io::Result<()> {
        self.0 += 1;
        Ok(())
    }

    fn record_list(&mut self, _: &[&dyn fmt::Display]) -> io::Result<()> {
        self.0 += 1;
        Ok(())
    }

    fn record_frame(&mut self, _: Location<'_>) -> io::Result<()> {
        self.0 += 1;
        Ok(())
    }

    fn record_diff(&mut self, _: &str, _: &str) -> io::Result<()> {
        self.0 += 1;
        Ok(())
    }

    fn record_backtrace(&mut self, _: &dyn fmt::Display, _: &Backtrace) -> io::Result<()> {
        self.0 += 1;
        Ok(())
    }

    fn record_command(&mut self, _: &str) -> io::Result<()> {
        self.0 += 1;
        Ok(())
    }

    fn record_group(&mut self, _: &dyn fmt::Display, _: &dyn Advices) -> io::Result<()> {
        self.0 += 1;
        Ok(())
    }
}

/// Implements [Advices] for verbose advices of a diagnostic.
struct PrintVerboseAdvices<'a, D: ?Sized>(&'a D);

impl<D: Diagnostic + ?Sized> Advices for PrintVerboseAdvices<'_, D> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        self.0.verbose_advices(visitor)
    }
}

/// Wrapper type over [fmt::Write] that injects `ident_text` at the start of
/// every line.
struct IndentWriter<'a, W: ?Sized> {
    writer: &'a mut W,
    pending_indent: bool,
    ident_text: &'static str,
}

impl<'write> IndentWriter<'write, dyn fmt::Write + 'write> {
    fn wrap<'slot, 'fmt: 'write + 'slot>(
        fmt: &'fmt mut fmt::Formatter<'_>,
        slot: &'slot mut Option<Self>,
        pending_indent: bool,
        ident_text: &'static str,
    ) -> fmt::Formatter<'slot> {
        fmt.wrap_writer(|writer| {
            slot.get_or_insert(Self {
                writer,
                pending_indent,
                ident_text,
            })
        })
    }
}

impl<W: fmt::Write + ?Sized> fmt::Write for IndentWriter<'_, W> {
    fn write_str(
        &mut self,
        elements: &fmt::MarkupElements<'_>,
        mut content: &str,
    ) -> io::Result<()> {
        while !content.is_empty() {
            if self.pending_indent {
                self.writer.write_str(elements, self.ident_text)?;
                self.pending_indent = false;
            }

            if let Some(index) = content.find('\n') {
                let (start, end) = content.split_at(index + 1);
                self.writer.write_str(elements, start)?;
                self.pending_indent = true;
                content = end;
            } else {
                return self.writer.write_str(elements, content);
            }
        }

        Ok(())
    }

    fn write_fmt(
        &mut self,
        elements: &fmt::MarkupElements<'_>,
        content: std::fmt::Arguments<'_>,
    ) -> io::Result<()> {
        if let Some(content) = content.as_str() {
            self.write_str(elements, content)
        } else {
            let content = content.to_string();
            self.write_str(elements, &content)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use rome_console::{fmt, markup};
    use rome_diagnostics::v2::{DiagnosticTags, Severity};
    use rome_diagnostics_categories::{category, Category};
    use rome_text_size::{TextRange, TextSize};
    use serde_json::{from_value, json};

    use crate::v2::{
        Advices, Diagnostic, FilePath, Location, LogCategory, PrintDiagnostic, Resource,
        SourceCode, Visit,
    };
    use crate::{self as rome_diagnostics};

    #[derive(Debug)]
    struct TestDiagnostic<A> {
        path: Option<String>,
        span: Option<TextRange>,
        source_code: Option<String>,
        advice: Option<A>,
        verbose_advice: Option<A>,
        source: Option<Box<dyn Diagnostic>>,
    }

    impl<A> TestDiagnostic<A> {
        fn empty() -> Self {
            Self {
                path: None,
                span: None,
                source_code: None,
                advice: None,
                verbose_advice: None,
                source: None,
            }
        }

        fn with_location() -> Self {
            Self {
                path: Some(String::from("path")),
                span: Some(TextRange::at(TextSize::from(0), TextSize::from(6))),
                source_code: Some(String::from("source code")),
                advice: None,
                verbose_advice: None,
                source: None,
            }
        }
    }

    impl<A: Advices + std::fmt::Debug> Diagnostic for TestDiagnostic<A> {
        fn category(&self) -> Option<&Category> {
            Some(category!("internalError/io"))
        }

        fn severity(&self) -> Severity {
            Severity::Error
        }

        fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(fmt, "diagnostic message")
        }

        fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
            write!(fmt, "diagnostic message")
        }

        fn advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            if let Some(advice) = &self.advice {
                advice.record(visitor)?;
            }

            Ok(())
        }

        fn verbose_advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            if let Some(advice) = &self.verbose_advice {
                advice.record(visitor)?;
            }

            Ok(())
        }

        fn location(&self) -> Option<Location<'_>> {
            Location::builder()
                .resource(&self.path)
                .span(&self.span)
                .source_code(&self.source_code)
                .build()
        }

        fn tags(&self) -> DiagnosticTags {
            DiagnosticTags::FIXABLE
        }

        fn source(&self) -> Option<&dyn Diagnostic> {
            self.source.as_deref()
        }
    }

    #[derive(Debug)]
    struct LogAdvices;

    impl Advices for LogAdvices {
        fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            visitor.record_log(LogCategory::Error, &"error")?;
            visitor.record_log(LogCategory::Warn, &"warn")?;
            visitor.record_log(LogCategory::Info, &"info")?;
            visitor.record_log(LogCategory::None, &"none")
        }
    }

    #[derive(Debug)]
    struct ListAdvice;

    impl Advices for ListAdvice {
        fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            visitor.record_list(&[&"item 1", &"item 2"])
        }
    }

    #[derive(Debug)]
    struct FrameAdvice;

    impl Advices for FrameAdvice {
        fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            visitor.record_frame(Location {
                resource: Resource::File(FilePath::Path("other_path")),
                span: Some(TextRange::new(TextSize::from(8), TextSize::from(16))),
                source_code: Some(SourceCode {
                    text: "context location context",
                    line_starts: None,
                }),
            })
        }
    }

    #[derive(Debug)]
    struct DiffAdvice;

    impl Advices for DiffAdvice {
        fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            visitor.record_diff("context before context", "context after context")
        }
    }

    #[derive(Debug)]
    struct BacktraceAdvice;

    impl Advices for BacktraceAdvice {
        fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            let backtrace = from_value(json!([
                {
                    "ip": 0x0f0f0f0f,
                    "symbols": [
                        {
                            "name": "crate::module::function",
                            "filename": "crate/src/module.rs",
                            "lineno": 8,
                            "colno": 16
                        }
                    ]
                }
            ]));

            visitor.record_backtrace(&"Backtrace Title", &backtrace.unwrap())
        }
    }

    #[derive(Debug)]
    struct CommandAdvice;

    impl Advices for CommandAdvice {
        fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            visitor.record_command("rome command --argument")
        }
    }

    #[derive(Debug)]
    struct GroupAdvice;

    impl Advices for GroupAdvice {
        fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            visitor.record_group(&"Group Title", &LogAdvices)
        }
    }

    #[test]
    fn test_header() {
        let diag = TestDiagnostic::<LogAdvices>::with_location();

        let diag = markup!({ PrintDiagnostic(&diag) }).to_owned();

        let expected = markup!{
            "path:1:1 internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            <Emphasis><Error>"  ✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
            "    "<Info>"┌─"</Info>" path:1:1\n"
            "    "<Info>"│"</Info>"\n"
            <Info>"  1"</Info>" "<Info>"│"</Info>" "<Error>"source"</Error>" code\n"
            "    "<Info>"│"</Info>" "<Error>"^^^^^^"</Error>"\n"
            "  \n"
        }.to_owned();

        assert_eq!(diag, expected);
    }
    #[test]
    fn test_log_advices() {
        let diag = TestDiagnostic {
            advice: Some(LogAdvices),
            ..TestDiagnostic::empty()
        };

        let diag = markup!({ PrintDiagnostic(&diag) }).to_owned();

        let expected = markup!{
            "internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            <Emphasis><Error>"  ✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
            <Emphasis><Error>"  ✖"</Error></Emphasis>" "<Error>"error"</Error>"\n"
            "  \n"
            <Emphasis><Warn>"  ⚠"</Warn></Emphasis>" "<Warn>"warn"</Warn>"\n"
            "  \n"
            <Emphasis><Info>"  ℹ"</Info></Emphasis>" "<Info>"info"</Info>"\n"
            "  \n"
            "  none\n"
            "  \n"
        }.to_owned();

        assert_eq!(diag, expected);
    }

    #[test]
    fn test_list_advice() {
        let diag = TestDiagnostic {
            advice: Some(ListAdvice),
            ..TestDiagnostic::empty()
        };

        let diag = markup!({ PrintDiagnostic(&diag) }).to_owned();

        let expected = markup!{
            "internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            <Emphasis><Error>"  ✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
            "  - item 1\n"
            "  - item 2\n"
            "  \n"
        }.to_owned();

        assert_eq!(diag, expected);
    }

    #[test]
    fn test_frame_advice() {
        let diag = TestDiagnostic {
            advice: Some(FrameAdvice),
            ..TestDiagnostic::empty()
        };

        let diag = markup!({ PrintDiagnostic(&diag) }).to_owned();

        let expected = markup!{
            "internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            <Emphasis><Error>"  ✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
                           "    "<Info>"┌─"</Info>" other_path:1:9\n"
                           "    "<Info>"│"</Info>"\n"
            <Info>"  1"</Info>" "<Info>"│"</Info>" context "<Error>"location"</Error>" context\n"
                           "    "<Info>"│"</Info>"         "<Error>"^^^^^^^^"</Error>"\n"
            "  \n"
        }.to_owned();

        assert_eq!(diag, expected);
    }

    #[test]
    fn test_diff_advice() {
        let diag = TestDiagnostic {
            advice: Some(DiffAdvice),
            ..TestDiagnostic::empty()
        };

        let diag = markup!({ PrintDiagnostic(&diag) }).to_owned();

        let expected = markup!{
            "internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            <Emphasis><Error>"  ✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
            "      | "<Info>"@@ -1 +1 @@"</Info>"\n"
            "  0   | "<Error>"- context before context"</Error>"\n"
            "    0 | "<Success>"+ context after context"</Success>"\n"
            "  \n"
        }.to_owned();

        assert_eq!(diag, expected);
    }

    #[test]
    fn test_backtrace_advice() {
        let diag = TestDiagnostic {
            advice: Some(BacktraceAdvice),
            ..TestDiagnostic::empty()
        };

        let diag = markup!({ PrintDiagnostic(&diag) }).to_owned();

        let expected = markup!{
            "internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            <Emphasis><Error>"  ✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
            <Emphasis><Info>"  ℹ"</Info></Emphasis>" "<Info>"Backtrace Title"</Info>"\n"
            "  \n"
            "     0: crate::module::function\n"
            "            at crate/src/module.rs:8:16\n"
        }.to_owned();

        assert_eq!(diag, expected);
    }

    #[test]
    fn test_command_advice() {
        let diag = TestDiagnostic {
            advice: Some(CommandAdvice),
            ..TestDiagnostic::empty()
        };

        let diag = markup!({ PrintDiagnostic(&diag) }).to_owned();

        let expected = markup!{
            "internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            <Emphasis><Error>"  ✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
            <Emphasis>"  $"</Emphasis>" rome command --argument\n"
            "  \n"
        }.to_owned();

        assert_eq!(diag, expected);
    }

    #[test]
    fn test_group_advice() {
        let diag = TestDiagnostic {
            advice: Some(GroupAdvice),
            ..TestDiagnostic::empty()
        };

        let diag = markup!({ PrintDiagnostic(&diag) }).to_owned();

        let expected = markup!{
            "internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            <Emphasis><Error>"  ✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
            <Emphasis>"  Group Title"</Emphasis>"\n"
            "  \n"
            <Emphasis><Error>"    ✖"</Error></Emphasis>" "<Error>"error"</Error>"\n"
            "    \n"
            <Emphasis><Warn>"    ⚠"</Warn></Emphasis>" "<Warn>"warn"</Warn>"\n"
            "    \n"
            <Emphasis><Info>"    ℹ"</Info></Emphasis>" "<Info>"info"</Info>"\n"
            "    \n"
            "    none\n"
            "    \n"
        }.to_owned();

        assert_eq!(diag, expected);
    }
}
