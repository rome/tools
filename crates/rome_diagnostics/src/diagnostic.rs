use crate::{
    file::{FileId, FileSpan, Span},
    v2::Category,
    Applicability, CodeSuggestion, DiagnosticTag,
};
use rome_console::fmt::Display;
use rome_console::{markup, MarkupBuf};
use rome_text_edit::TextEdit;
use serde::{Deserialize, Serialize};

/// A diagnostic message that can give information
/// like errors or warnings.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Diagnostic {
    pub file_id: FileId,

    pub severity: Severity,
    pub code: Option<&'static Category>,
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
    pub fn error(file_id: FileId, code: &'static Category, title: impl Display) -> Self {
        Self::new_with_code(file_id, Severity::Error, title, Some(code))
    }

    /// Creates a new [`Diagnostic`] with the `Warning` severity.
    pub fn warning(file_id: FileId, code: &'static Category, title: impl Display) -> Self {
        Self::new_with_code(file_id, Severity::Warning, title, Some(code))
    }

    /// Creates a new [`Diagnostic`] with the `Help` severity.
    pub fn help(file_id: FileId, code: &'static Category, title: impl Display) -> Self {
        Self::new_with_code(file_id, Severity::Help, title, Some(code))
    }

    /// Creates a new [`Diagnostic`] with the `Note` severity.
    pub fn note(file_id: FileId, code: &'static Category, title: impl Display) -> Self {
        Self::new_with_code(file_id, Severity::Note, title, Some(code))
    }

    /// Creates a new [`Diagnostic`] with the `Bug` severity.
    pub fn bug(file_id: FileId, code: &'static Category, title: impl Display) -> Self {
        Self::new_with_code(file_id, Severity::Bug, title, Some(code))
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
        code: Option<&'static Category>,
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

    /// Add a suggestion to this `Diagnostic`
    pub fn suggestion_full(
        mut self,
        span: impl Span,
        msg: impl Display,
        suggestion: TextEdit,
        applicability: Applicability,
    ) -> Self {
        let file = self.file_id;
        let span = FileSpan {
            file,
            range: span.as_range(),
        };
        let suggestion = CodeSuggestion {
            applicability,
            msg: markup!({ msg }).to_owned(),
            labels: vec![],
            span,
            suggestion,
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

/// A severity level for diagnostic messages.
///
/// These are ordered in the following way:
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Severity {
    /// A help message.
    Help,
    /// A note.
    Note,
    /// A warning.
    Warning,
    /// An error.
    Error,
    /// An unexpected bug.
    Bug,
}

impl From<Severity> for &'static str {
    fn from(level: Severity) -> Self {
        match level {
            Severity::Bug => "bug",
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Help => "help",
            Severity::Note => "note",
        }
    }
}

/// Everything that can be added to a diagnostic, like
/// a suggestion that will be displayed under the actual error.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SubDiagnostic {
    pub severity: Severity,
    pub msg: MarkupBuf,
    pub span: FileSpan,
}

/// A note or help that is displayed under the diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Footer {
    pub msg: MarkupBuf,
    pub severity: Severity,
}
