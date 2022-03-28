use crate::suggestion::SuggestionChange;
use crate::{
    file::{FileId, FileSpan, Span},
    Applicability, CodeSuggestion, DiagnosticTag, Severity, SuggestionStyle,
};
use rome_console::fmt::Display;
use rome_console::{markup, MarkupBuf};
use rome_text_edit::*;

/// A diagnostic message that can give information
/// like errors or warnings.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Diagnostic {
    pub file_id: FileId,

    pub severity: Severity,
    pub code: Option<MarkupBuf>,
    pub title: MarkupBuf,
    pub summary: Option<String>,
    pub tag: Option<DiagnosticTag>,

    pub primary: Option<SubDiagnostic>,
    pub children: Vec<SubDiagnostic>,
    pub suggestions: Vec<CodeSuggestion>,
    pub footers: Vec<Footer>,
}

impl Diagnostic {
    /// Creates a new [`Diagnostic`] with the `Error` severity.
    pub fn error(file_id: FileId, code: impl Display, title: impl Display) -> Self {
        Self::new_with_code(
            file_id,
            Severity::Error,
            title,
            Some(markup!({ code }).to_owned()),
        )
    }

    /// Creates a new [`Diagnostic`] with the `Warning` severity.
    pub fn warning(file_id: FileId, code: impl Display, title: impl Display) -> Self {
        Self::new_with_code(
            file_id,
            Severity::Warning,
            title,
            Some(markup!({ code }).to_owned()),
        )
    }

    /// Creates a new [`Diagnostic`] with the `Help` severity.
    pub fn help(file_id: FileId, code: impl Display, title: impl Display) -> Self {
        Self::new_with_code(
            file_id,
            Severity::Help,
            title,
            Some(markup!({ code }).to_owned()),
        )
    }

    /// Creates a new [`Diagnostic`] with the `Note` severity.
    pub fn note(file_id: FileId, code: impl Display, title: impl Display) -> Self {
        Self::new_with_code(
            file_id,
            Severity::Note,
            title,
            Some(markup!({ code }).to_owned()),
        )
    }

    /// Creates a new [`Diagnostic`] that will be used in a builder-like way
    /// to modify labels, and suggestions.
    pub fn new(file_id: FileId, severity: Severity, title: impl Display) -> Self {
        Self::new_with_code(file_id, severity, title, None)
    }

    /// Creates a new [`Diagnostic`] with an error code that will be used in a builder-like way
    /// to modify labels, and suggestions.
    pub fn new_with_code(
        file_id: FileId,
        severity: Severity,
        title: impl Display,
        code: Option<MarkupBuf>,
    ) -> Self {
        Self {
            file_id,
            code,
            severity,
            title: markup!({ title }).to_owned(),
            summary: None,
            primary: None,
            tag: None,
            children: vec![],
            suggestions: vec![],
            footers: vec![],
        }
    }

    /// Overwrites the severity of this diagnostic.
    pub fn severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    /// Set an explicit plain-text summary for this diagnostic.
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    /// Marks this diagnostic as deprecated code, which will
    /// be displayed in the language server.
    ///
    /// This does not have any influence on the diagnostic rendering.
    pub fn deprecated(mut self) -> Self {
        self.tag = if matches!(self.tag, Some(DiagnosticTag::Unnecessary)) {
            Some(DiagnosticTag::Both)
        } else {
            Some(DiagnosticTag::Deprecated)
        };
        self
    }

    /// Marks this diagnostic as unnecessary code, which will
    /// be displayed in the language server.
    ///
    /// This does not have any influence on the diagnostic rendering.
    pub fn unnecessary(mut self) -> Self {
        self.tag = if matches!(self.tag, Some(DiagnosticTag::Deprecated)) {
            Some(DiagnosticTag::Both)
        } else {
            Some(DiagnosticTag::Unnecessary)
        };
        self
    }

    /// Attaches a label to this [`Diagnostic`], that will point to another file
    /// that is provided.
    pub fn label_in_file(mut self, severity: Severity, span: FileSpan, msg: impl Display) -> Self {
        self.children.push(SubDiagnostic {
            severity,
            msg: markup!({ msg }).to_owned(),
            span,
        });
        self
    }

    /// Attaches a label to this [`Diagnostic`].
    ///
    /// The given span has to be in the file that was provided while creating this [`Diagnostic`].
    pub fn label(mut self, severity: Severity, span: impl Span, msg: impl Display) -> Self {
        self.children.push(SubDiagnostic {
            severity,
            msg: markup!({ msg }).to_owned(),
            span: FileSpan::new(self.file_id, span),
        });
        self
    }

    /// Attaches a primary label to this [`Diagnostic`].
    pub fn primary(mut self, span: impl Span, msg: impl Display) -> Self {
        self.primary = Some(SubDiagnostic {
            severity: self.severity,
            msg: markup!({ msg }).to_owned(),
            span: FileSpan::new(self.file_id, span),
        });
        self
    }

    /// Attaches a secondary label to this [`Diagnostic`].
    pub fn secondary(self, span: impl Span, msg: impl Display) -> Self {
        self.label(Severity::Note, span, msg)
    }

    /// Prints out a message that suggests a possible solution, that is in another
    /// file as this `Diagnostic`, to the error.
    ///
    /// If the message plus the suggestion is longer than 25 chars,
    /// the suggestion is displayed as a new children of this `Diagnostic`,
    /// otherwise it will be inlined with the other labels.
    ///
    /// A suggestion is displayed like:
    /// ```no_rust
    /// try adding a `;`: console.log();
    /// ```
    /// or in a separate multiline suggestion
    ///
    /// The message should not contain the `:` because it's added automatically.
    /// The suggestion will automatically be wrapped inside two backticks.
    pub fn suggestion_in_file(
        self,
        span: impl Span,
        msg: impl Display,
        suggestion: impl Into<String>,
        applicability: Applicability,
        file: FileId,
    ) -> Self {
        self.suggestion_inner(
            span,
            markup!({ msg }).to_owned(),
            suggestion,
            applicability,
            None,
            file,
        )
    }

    fn auto_suggestion_style(span: &impl Span, msg: &MarkupBuf) -> SuggestionStyle {
        if span.as_range().len() + msg.len() > TextSize::from(25u32) {
            SuggestionStyle::Full
        } else {
            SuggestionStyle::Inline
        }
    }

    /// Prints out a message that suggests a possible solution to the error.
    ///
    /// If the message plus the suggestion is longer than 25 chars,
    /// the suggestion is displayed as a new children of this `Diagnostic`,
    /// otherwise it will be inlined with the other labels.
    ///
    /// A suggestion is displayed like:
    /// ```no_rust
    /// try adding a `;`: console.log();
    /// ```
    /// or in a separate multiline suggestion
    ///
    /// The message should not contain the `:` because it's added automatically.
    /// The suggestion will automatically be wrapped inside two backticks.
    pub fn suggestion(
        self,
        span: impl Span,
        msg: impl Display,
        suggestion: impl Into<String>,
        applicability: Applicability,
    ) -> Self {
        let file = self.file_id;
        self.suggestion_inner(
            span,
            markup!({ msg }).to_owned(),
            suggestion,
            applicability,
            None,
            file,
        )
    }

    /// Add a suggestion which is always shown in the [Full](SuggestionStyle::Full) style.
    pub fn suggestion_full(
        self,
        span: impl Span,
        msg: impl Display,
        suggestion: impl Into<String>,
        applicability: Applicability,
    ) -> Self {
        let file = self.file_id;
        self.suggestion_inner(
            span,
            markup!({ msg }).to_owned(),
            suggestion,
            applicability,
            SuggestionStyle::Full,
            file,
        )
    }

    /// Add a suggestion which is always shown in the [Inline](SuggestionStyle::Inline) style.
    pub fn suggestion_inline(
        self,
        span: impl Span,
        msg: impl Display,
        suggestion: impl Into<String>,
        applicability: Applicability,
    ) -> Self {
        let file = self.file_id;
        self.suggestion_inner(
            span,
            markup!({ msg }).to_owned(),
            suggestion,
            applicability,
            SuggestionStyle::Inline,
            file,
        )
    }

    /// Add a suggestion which does not have a suggestion code.
    pub fn suggestion_no_code(
        self,
        span: impl Span,
        msg: impl Display,
        applicability: Applicability,
    ) -> Self {
        let file = self.file_id;
        self.suggestion_inner(
            span,
            markup!({ msg }).to_owned(),
            "",
            applicability,
            SuggestionStyle::HideCode,
            file,
        )
    }

    pub fn indel_suggestion(
        mut self,
        indels: impl IntoIterator<Item = Indel>,
        span: impl Span,
        msg: impl Display,
        applicability: Applicability,
    ) -> Self {
        let span = FileSpan {
            file: self.file_id,
            range: span.as_range(),
        };
        let indels = indels.into_iter().collect::<Vec<_>>();
        let labels = indels
            .iter()
            .filter(|x| !x.insert.is_empty())
            .map(|x| {
                TextRange::new(
                    x.delete.as_range().start(),
                    x.delete.as_range().start() + TextSize::of(&x.insert),
                )
            })
            .collect();

        let suggestion = CodeSuggestion {
            substitution: SuggestionChange::Indels(indels),
            applicability,
            msg: markup!({ msg }).to_owned(),
            labels,
            span,
            style: SuggestionStyle::Full,
        };
        self.suggestions.push(suggestion);
        self
    }

    /// Add a suggestion with info labels which point to places in the suggestion.
    ///
    /// **The label ranges are relative to the start of the span, not relative to the original code**
    pub fn suggestion_with_labels(
        mut self,
        span: impl Span,
        msg: impl Display,
        suggestion: impl Into<String>,
        applicability: Applicability,
        labels: impl IntoIterator<Item = impl Span>,
    ) -> Self {
        let span = FileSpan {
            file: self.file_id,
            range: span.as_range(),
        };

        let labels = labels
            .into_iter()
            .map(|x| {
                let range = x.as_range();
                TextRange::new(
                    span.range.start() + range.start(),
                    span.range.start() + range.end(),
                )
            })
            .collect::<Vec<_>>();
        let suggestion = CodeSuggestion {
            substitution: SuggestionChange::String(suggestion.into()),
            applicability,
            msg: markup!({ msg }).to_owned(),
            labels,
            span,
            style: SuggestionStyle::Full,
        };
        self.suggestions.push(suggestion);
        self
    }

    /// Add a suggestion with info labels which point to places in the suggestion.
    ///
    /// **The label ranges are relative to the source code, not relative to the original code**
    pub fn suggestion_with_src_labels(
        mut self,
        span: impl Span,
        msg: impl Display,
        suggestion: impl Into<String>,
        applicability: Applicability,
        labels: impl IntoIterator<Item = impl Span>,
    ) -> Self {
        let span = FileSpan {
            file: self.file_id,
            range: span.as_range(),
        };

        let labels = labels.into_iter().map(|x| x.as_range()).collect::<Vec<_>>();
        let suggestion = CodeSuggestion {
            substitution: SuggestionChange::String(suggestion.into()),
            applicability,
            msg: markup!({ msg }).to_owned(),
            labels,
            span,
            style: SuggestionStyle::Full,
        };
        self.suggestions.push(suggestion);
        self
    }

    fn suggestion_inner(
        mut self,
        span: impl Span,
        msg: MarkupBuf,
        suggestion: impl Into<String>,
        applicability: Applicability,
        style: impl Into<Option<SuggestionStyle>>,
        file: FileId,
    ) -> Self {
        let style = style
            .into()
            .unwrap_or_else(|| Self::auto_suggestion_style(&span, &msg));
        let span = FileSpan {
            file,
            range: span.as_range(),
        };
        let suggestion = CodeSuggestion {
            substitution: SuggestionChange::String(suggestion.into()),
            applicability,
            msg,
            labels: vec![],
            span,
            style,
        };
        self.suggestions.push(suggestion);
        self
    }

    /// Adds a footer to this `Diagnostic`, which will be displayed under the actual error.
    pub fn footer(mut self, severity: Severity, msg: impl Display) -> Self {
        self.footers.push(Footer {
            msg: markup!({ msg }).to_owned(),
            severity,
        });
        self
    }

    /// Adds a footer to this `Diagnostic`, with the `Help` severity.
    pub fn footer_help(self, msg: impl Display) -> Self {
        self.footer(Severity::Help, msg)
    }

    /// Adds a footer to this `Diagnostic`, with the `Note` severity.
    pub fn footer_note(self, msg: impl Display) -> Self {
        self.footer(Severity::Note, msg)
    }

    /// Checks if the severity of the current diagnostic is [Severity::Error] or higher
    pub fn is_error(&self) -> bool {
        self.severity >= Severity::Error
    }
}

/// Everything that can be added to a diagnostic, like
/// a suggestion that will be displayed under the actual error.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct SubDiagnostic {
    pub severity: Severity,
    pub msg: MarkupBuf,
    pub span: FileSpan,
}

/// A note or help that is displayed under the diagnostic.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Footer {
    pub msg: MarkupBuf,
    pub severity: Severity,
}
