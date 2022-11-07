//! Implementation of converting, and emitting diagnostics
//! into the console

use crate::file::FileSpan;
use crate::v2::advice::{LogCategory, Visit};
use crate::v2::{
    self, advice, Category, DiagnosticTags, FilePath, Location, PrintDiagnostic, Resource,
};
use crate::{file::Files, Diagnostic};
use crate::{Applicability, Severity, SubDiagnostic};
use rome_console::{
    fmt::{Display, Formatter, Termcolor},
    markup,
};
use std::fmt::Debug;
use std::io;
use termcolor::{ColorChoice, NoColor, StandardStream, WriteColor};

/// The emitter is responsible for emitting
/// diagnostics to a given output.
pub struct Emitter<'files> {
    files: &'files dyn Files,
}

impl<'files> Emitter<'files> {
    /// Creates a new `Emitter`.
    pub fn new(files: &'files dyn Files) -> Self {
        Self { files }
    }
}

impl Emitter<'_> {
    /// Render and emit the diagnostic to stderr
    ///
    /// This method will lock stderr for the entire time it takes to emit the diagnostic.
    pub fn emit_stderr(&mut self, d: &Diagnostic, color: bool) -> io::Result<()> {
        let out = StandardStream::stderr(if color {
            ColorChoice::Always
        } else {
            ColorChoice::Never
        });
        let mut out = out.lock();
        self.emit_with_writer(d, &mut out)
    }

    pub fn emit_with_writer(
        &mut self,
        d: &Diagnostic,
        writer: &mut dyn WriteColor,
    ) -> io::Result<()> {
        Formatter::new(&mut Termcolor(writer)).write_markup(markup! {
            {d.display(self.files)}
        })
    }
}

impl Diagnostic {
    pub fn as_diagnostic<'a>(&'a self, files: &'a dyn Files) -> impl v2::Diagnostic + 'a {
        DiagnosticPrinter {
            files,
            d: self,
            include_source: false,
        }
    }

    pub fn display<'a>(&'a self, files: &'a dyn Files) -> impl Display + 'a {
        DiagnosticPrinter {
            files,
            d: self,
            include_source: true,
        }
    }
}

#[derive(Clone, Copy)]
struct DiagnosticPrinter<'a> {
    files: &'a dyn Files,
    d: &'a Diagnostic,
    include_source: bool,
}

impl DiagnosticPrinter<'_> {
    fn lookup_location(&self, span: FileSpan) -> Option<Location<'_>> {
        let path = self.files.name(span.file)?;

        Some(Location {
            resource: Resource::File(FilePath::PathAndId {
                path,
                file_id: span.file,
            }),
            span: Some(span.range),
            source_code: if self.include_source {
                self.files.source(span.file)
            } else {
                None
            },
        })
    }

    fn record_label(&self, visitor: &mut dyn Visit, label: &SubDiagnostic) -> io::Result<()> {
        if !label.msg.is_empty() {
            visitor.record_log(map_severity_to_log_category(label.severity), &label.msg)?;
        }

        if let Some(location) = self.lookup_location(label.span) {
            visitor.record_frame(location)?;
        }

        Ok(())
    }
}

fn map_severity_to_log_category(severity: Severity) -> LogCategory {
    match severity {
        Severity::Help => LogCategory::Info,
        Severity::Note => LogCategory::Info,
        Severity::Warning => LogCategory::Warn,
        Severity::Error => LogCategory::Error,
        Severity::Bug => LogCategory::Error,
    }
}

impl Debug for DiagnosticPrinter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DiagnosticPrinter")
            .field("d", &self.d)
            .finish()
    }
}

impl<'a> Display for DiagnosticPrinter<'a> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> io::Result<()> {
        PrintDiagnostic(self).fmt(fmt)?;
        Ok(())
    }
}

impl v2::Diagnostic for DiagnosticPrinter<'_> {
    fn category(&self) -> Option<&'static Category> {
        self.d.code
    }

    fn severity(&self) -> v2::Severity {
        match self.d.severity {
            Severity::Help => v2::Severity::Hint,
            Severity::Note => v2::Severity::Information,
            Severity::Warning => v2::Severity::Warning,
            Severity::Error | Severity::Bug => v2::Severity::Error,
        }
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(summary) = &self.d.summary {
            fmt.write_str(summary)
        } else {
            let mut message = Termcolor(NoColor::new(Vec::new()));
            Display::fmt(&self.d.title, &mut Formatter::new(&mut message))
                .map_err(|_| std::fmt::Error)?;

            let message = message.0.into_inner();
            let message: String = String::from_utf8(message).map_err(|_| std::fmt::Error)?;
            fmt.write_str(&message)
        }
    }

    fn message(&self, fmt: &mut Formatter<'_>) -> io::Result<()> {
        fmt.write_markup(markup! {
            {self.d.title}
        })
    }

    fn advices(&self, visitor: &mut dyn advice::Visit) -> io::Result<()> {
        if let Some(label) = &self.d.primary {
            self.record_label(visitor, label)?;
        }

        for label in &self.d.children {
            self.record_label(visitor, label)?;
        }

        for footer in &self.d.footers {
            visitor.record_log(map_severity_to_log_category(footer.severity), &footer.msg)?;
        }

        for suggestion in &self.d.suggestions {
            let applicability = match suggestion.applicability {
                Applicability::Always => "Safe fix",
                Applicability::MaybeIncorrect => "Suggested fix",
            };

            visitor.record_log(
                LogCategory::Info,
                &markup! {
                    {applicability}": "{suggestion.msg}
                },
            )?;

            visitor.record_diff(&suggestion.suggestion)?;
        }

        Ok(())
    }

    fn location(&self) -> Option<Location<'_>> {
        let path = self.files.name(self.d.file_id)?;

        Some(Location {
            resource: Resource::File(FilePath::PathAndId {
                path,
                file_id: self.d.file_id,
            }),
            span: self.d.primary.as_ref().map(|label| label.span.range),
            source_code: if self.include_source {
                self.files.source(self.d.file_id)
            } else {
                None
            },
        })
    }

    fn tags(&self) -> DiagnosticTags {
        let mut tags = DiagnosticTags::empty();

        if !self.d.suggestions.is_empty() {
            tags |= DiagnosticTags::FIXABLE;
        }

        if self.d.severity == Severity::Bug {
            tags |= DiagnosticTags::INTERNAL;
        }

        if self.d.tag.map_or(false, |tag| tag.is_deprecated()) {
            tags |= DiagnosticTags::DEPRECATED_CODE;
        }

        if self.d.tag.map_or(false, |tag| tag.is_unnecessary()) {
            tags |= DiagnosticTags::UNNECESSARY_CODE;
        }

        tags
    }
}

#[cfg(test)]
mod tests {
    use rome_console::{markup, BufferConsole, ConsoleExt};
    use rome_text_edit::TextEdit;
    use rome_text_size::{TextRange, TextSize};

    use crate::{
        file::SimpleFile,
        v2::{category, FileId},
        Applicability, Diagnostic, Severity,
    };

    #[test]
    fn test_error_diagnostic() {
        const SOURCE: &str = "Lorem ipsum dolor sit amet,
consectetur adipiscing elit,
sed do eiusmod tempor incididunt ut
labore et dolore magna aliqua";

        let expected = markup! {
            "file_name "<Hyperlink href="https://docs.rome.tools/lint/rules/noArguments">"lint/correctness/noArguments"</Hyperlink>" "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            <Emphasis><Error>"  ✖"</Error></Emphasis>" "<Error>"message"</Error>"\n"
            "  \n"
            <Emphasis><Error>"  ✖"</Error></Emphasis>" "<Error>"label"</Error>"\n"
            "  \n"
            "    "<Emphasis>"1 │ "</Emphasis>"Lorem ipsum dolor sit amet,\n"
            <Emphasis><Error>"  >"</Error></Emphasis>" "<Emphasis>"2 │ "</Emphasis>"consectetur adipiscing elit,\n"
            "   "<Emphasis>"   │ "</Emphasis>"            "<Emphasis><Error>"^^^^^^^^^^^^^^^"</Error></Emphasis>"\n"
            "    "<Emphasis>"3 │ "</Emphasis>"sed do eiusmod tempor incididunt ut\n"
            "    "<Emphasis>"4 │ "</Emphasis>"labore et dolore magna aliqua\n"
            "  \n"
            <Emphasis><Info>"  ℹ"</Info></Emphasis>" "<Info>"footer note"</Info>"\n"
            "  \n"
            <Emphasis><Info>"  ℹ"</Info></Emphasis>" "<Info>"footer help"</Info>"\n"
            "  \n"
            <Emphasis><Info>"  ℹ"</Info></Emphasis>" "<Info>"Safe fix: suggestion"</Info>"\n"
            "  \n"
            "    "<Emphasis>"1"</Emphasis>" "<Emphasis>"1 │ "</Emphasis>"  Lorem ipsum dolor sit amet,\n"
            "    "<Emphasis>"2"</Emphasis>"  "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" "<Error>"consectetur"</Error><Error><Dim>"·"</Dim></Error><Error><Emphasis>"adipiscing"</Emphasis></Error><Error><Dim>"·"</Dim></Error><Error><Emphasis>"elit"</Emphasis></Error><Error>","</Error>"\n"
            "      "<Emphasis>"2 │ "</Emphasis><Success>"+"</Success>" "<Success>"consectetur"</Success><Success><Dim>"·"</Dim></Success><Success><Emphasis>"completely"</Emphasis></Success><Success><Dim>"·"</Dim></Success><Success><Emphasis>"different"</Emphasis></Success>"\n"
            "      "<Emphasis>"3 │ "</Emphasis><Success>"+"</Success>" "<Success><Emphasis>"text"</Emphasis></Success><Success>","</Success>"\n"
            "    "<Emphasis>"3"</Emphasis>" "<Emphasis>"4 │ "</Emphasis>"  sed do eiusmod tempor incididunt ut\n"
            "    "<Emphasis>"4"</Emphasis>" "<Emphasis>"5 │ "</Emphasis>"  labore et dolore magna aliqua\n"
            "  \n"
        }.to_owned();

        let files = SimpleFile::new(String::from("file_name"), SOURCE.into());

        let mut fixed_code = SOURCE.to_string();
        fixed_code.replace_range(40..55, "completely different\ntext");

        let diag = Diagnostic::error(
            FileId::zero(),
            category!("lint/correctness/noArguments"),
            "message",
        )
        .label(
            Severity::Error,
            TextRange::new(TextSize::from(40u32), TextSize::from(55u32)),
            "label",
        )
        .suggestion_full(
            TextRange::new(TextSize::from(40u32), TextSize::from(55u32)),
            "suggestion",
            TextEdit::from_unicode_words(SOURCE, &fixed_code),
            Applicability::Always,
        )
        .footer_note("footer note")
        .footer_help("footer help");

        let mut console = BufferConsole::default();
        console.log(markup! {
            {diag.display(&files)}
        });

        let mut iter = console.out_buffer.into_iter();

        let message = match iter.next() {
            Some(msg) => msg,
            other => panic!("unexpected message {other:?}"),
        };

        assert_eq!(
            message.content, expected,
            "\nactual:\n{:#?}\nexpected:\n{expected:#?}",
            message.content
        );

        assert!(iter.next().is_none());
    }
}
