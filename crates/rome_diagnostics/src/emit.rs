//! Implementation of converting, and emitting diagnostics
//! using `codespan`.

use crate::{file::Files, Diagnostic};
use crate::{SuggestionChange, SuggestionStyle};
use rome_console::{
    codespan::{Codespan, Label, LabelStyle, Locus, Severity, WithSeverity},
    diff::{Diff, DiffMode},
    fmt::{Display, Formatter, Termcolor},
    markup, MarkupBuf,
};
use rome_rowan::{TextRange, TextSize};
use rome_text_edit::apply_indels;
use std::borrow::Cow;
use std::io;
use termcolor::{ColorChoice, StandardStream, WriteColor};

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
    pub fn display<'a>(&'a self, files: &'a dyn Files) -> impl Display + 'a {
        DiagnosticPrinter { files, d: self }
    }
}

#[derive(Clone, Copy)]
struct DiagnosticPrinter<'a> {
    files: &'a dyn Files,
    d: &'a Diagnostic,
}

impl<'a> Display for DiagnosticPrinter<'a> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> io::Result<()> {
        let name = self
            .files
            .name(self.d.file_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "file not found"))?;
        let source_file = self
            .files
            .source(self.d.file_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "file not found"))?;

        let locus = if let Some(label) = &self.d.primary {
            Locus::FileLocation {
                name,
                location: source_file.location(label.span.range.start())?,
            }
        } else {
            Locus::File { name }
        };

        // If the diagnostic doesn't have a codespan, show the locus in the header instead
        let has_codespan = self.d.primary.is_some() || !self.d.children.is_empty();

        fmt.write_markup(markup! {
            {DiagnosticHeader {
                locus: if !has_codespan {
                    Some(locus)
                } else {
                    None
                },
                severity: self.d.severity,
                code: self.d.code.as_deref().filter(|code| !code.is_empty()),
                title: &self.d.title,
            }}
            "\n"
        })?;

        if has_codespan {
            let labels: Vec<_> = self
                .d
                .children
                .iter()
                .chain(self.d.primary.as_ref())
                .map(|sub| {
                    let style = if sub.severity == Severity::Bug || sub.severity == Severity::Error
                    {
                        LabelStyle::Primary
                    } else {
                        LabelStyle::Secondary
                    };

                    if sub.msg.is_empty() {
                        Label {
                            style,
                            range: sub.span.range,
                            message: MarkupBuf::default(),
                        }
                    } else {
                        Label {
                            style,
                            range: sub.span.range,
                            message: markup! {
                                {sub.msg}
                            }
                            .to_owned(),
                        }
                    }
                })
                .collect();

            fmt.write_markup(markup! {
                {Codespan { source_file, severity: self.d.severity, locus: Some(locus), labels: &labels }}"\n"
            })?;
        }

        for suggestion in &self.d.suggestions {
            match suggestion.style {
                SuggestionStyle::Full => {
                    let old = self
                        .files
                        .source(suggestion.span.file)
                        .expect("Non existant file id")
                        .source;

                    let range = suggestion.span.range;
                    let new = match &suggestion.substitution {
                        SuggestionChange::Indels(indels) => {
                            let mut new = String::from(&old[range]);
                            apply_indels(indels, &mut new);
                            Cow::Owned(new)
                        }
                        SuggestionChange::String(string) => Cow::Borrowed(string),
                    };

                    let new = format!(
                        "{}{}{}",
                        &old[TextRange::new(TextSize::from(0u32), range.start())],
                        new,
                        &old[TextRange::new(range.end(), TextSize::of(old))],
                    );

                    fmt.write_markup(markup! {
                        <Info>{suggestion.msg}</Info>"\n"
                        {Diff { mode: DiffMode::Unified, left: old, right: &new }}
                    })?;
                }
                SuggestionStyle::Inline => {
                    let replacement = match &suggestion.substitution {
                        SuggestionChange::Indels(indels) => {
                            let mut old = String::from(
                                &self
                                    .files
                                    .source(suggestion.span.file)
                                    .expect("Non existant file id")
                                    .source[suggestion.span.range],
                            );
                            apply_indels(indels, &mut old);
                            old
                        }
                        SuggestionChange::String(string) => string.clone(),
                    };

                    fmt.write_markup(markup! {
                        <Info>{suggestion.msg}": `"{replacement}"`"</Info>"\n"
                    })?;
                }
                SuggestionStyle::HideCode => {
                    fmt.write_markup(markup! {
                        <Info>{suggestion.msg}</Info>"\n"
                    })?;
                }
                SuggestionStyle::DontShow => {}
            }
        }

        if !self.d.suggestions.is_empty() {
            writeln!(fmt)?;
        }

        for footer in &self.d.footers {
            let level = match footer.severity {
                Severity::Note => {
                    fmt.write_markup(markup! {
                        "=  note: "{footer.msg}"\n"
                    })?;
                    continue;
                }
                level => <&str>::from(level),
            };

            fmt.write_markup(markup! {
                "= "
                {WithSeverity(LabelStyle::Primary, footer.severity, &markup! {
                    {level}": "
                })}
                {footer.msg}"\n"
            })?;
        }

        if !self.d.footers.is_empty() {
            writeln!(fmt)?;
        }

        Ok(())
    }
}

pub struct DiagnosticHeader<'a> {
    pub locus: Option<Locus<'a>>,
    pub severity: Severity,
    pub code: Option<&'a str>,
    pub title: &'a str,
}

impl<'a> Display for DiagnosticHeader<'a> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> io::Result<()> {
        if let Some(locus) = &self.locus {
            fmt.write_markup(markup! {
                {locus}": "
            })?;
        }

        let level = <&str>::from(self.severity);

        match self.code {
            Some(code) => fmt.write_markup(markup! {
                {WithSeverity(LabelStyle::Primary, self.severity, &markup!{ {level}"["{code}"]" })}
            })?,
            None => fmt.write_markup(markup! {
                {WithSeverity(LabelStyle::Primary, self.severity, &level)}
            })?,
        }

        fmt.write_markup(markup! {
            <Emphasis>": "{self.title}</Emphasis>
        })
    }
}

#[cfg(test)]
mod tests {
    use rome_console::{codespan::Severity, markup, BufferConsole, ConsoleExt, Markup};
    use rome_rowan::{TextRange, TextSize};

    use crate::{file::SimpleFile, Applicability, Diagnostic};

    #[test]
    fn test_error_diagnostic() {
        const SOURCE: &str = "Lorem ipsum dolor sit amet,
consectetur adipiscing elit,
sed do eiusmod tempor incididunt ut
labore et dolore magna aliqua";

        const DIAGNOSTIC: Markup<'static> = markup! {
            <Error>"error[CODE]"</Error><Emphasis>": message"</Emphasis>"\n"
            "  "<Info>"┌─"</Info>" file_name\n"
            "  "<Info>"│"</Info>"\n"
                <Info>"2"</Info>" "<Info>"│"</Info>" consectetur "<Error>"adipiscing elit"</Error>",\n"
            "  "<Info>"│"</Info>                   "             "<Error>"^^^^^^^^^^^^^^^"</Error>" "<Error>"label"</Error>"\n"
            "\n"
            <Info>"suggestion"</Info>"\n"
            "    | "<Info>"@@ -1,4 +1,5 @@"</Info>"\n"
            "0 0 |   Lorem ipsum dolor sit amet,\n"
            "1   | "<Error>"- consectetur adipiscing elit,"</Error>"\n"
            "  1 | "<Success>"+ consectetur completely different"</Success>"\n"
            "  2 | "<Success>"+ text,"</Success>"\n"
            "2 3 |   sed do eiusmod tempor incididunt ut\n"
            "3 4 |   labore et dolore magna aliqua\n"
            "\n"
            "=  note: footer note\n"
            "= "<Info>"help: "</Info>"footer help\n"
            "\n"
        };

        let files = SimpleFile::new(String::from("file_name"), SOURCE.into());

        let diag = Diagnostic::error(0, "CODE", "message")
            .label(
                Severity::Error,
                TextRange::new(TextSize::from(40u32), TextSize::from(55u32)),
                "label",
            )
            .suggestion_full(
                TextRange::new(TextSize::from(40u32), TextSize::from(55u32)),
                "suggestion",
                "completely different\ntext",
                Applicability::Always,
            )
            .footer_note("footer note")
            .footer_help("footer help");

        let mut console = BufferConsole::default();
        console.log(markup! {
            {diag.display(&files)}
        });

        let mut iter = console.buffer.into_iter();

        let message = match iter.next() {
            Some(msg) => msg,
            other => panic!("unexpected message {other:?}"),
        };

        assert_eq!(message.content, DIAGNOSTIC.to_owned());

        assert!(iter.next().is_none());
    }
}
