pub mod formatter;

use crate::reports::formatter::{FormatterReportFileDetail, FormatterReportSummary};
use formatter::FormatterReport;
use rome_diagnostics::{Category, Severity};
use rome_service::RomeError;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Default, Serialize)]
pub struct Report {
    /// Information related to the formatter
    formatter: FormatterReport,

    /// Diagnostics tracked during a generic traversal
    ///
    /// The key is the path of the file where the diagnostics occurred
    diagnostics: HashMap<String, ReportErrorKind>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReportErrorKind {
    Diagnostic(ReportDiagnostic),
    Diff(ReportDiff),
}

/// Information computed from a [diagnostic][rome_diagnostics::Diagnostic]
#[derive(Debug, Serialize)]
pub struct ReportDiagnostic {
    /// Severity of the [diagnostic][rome_diagnostics::Diagnostic]
    pub severity: Severity,
    /// The code of the [diagnostic][rome_diagnostics::Diagnostic]
    pub code: Option<&'static Category>,
    /// The title of the [diagnostic][rome_diagnostics::Diagnostic]
    pub title: String,
}

/// Information computed from a diff result
#[derive(Debug, Serialize)]
pub struct ReportDiff {
    /// The severity fo the diff
    pub severity: Severity,
    /// How was the code before the command
    pub before: String,
    /// How is the code after the command
    pub after: String,
}

impl Default for ReportDiagnostic {
    fn default() -> Self {
        Self {
            severity: Severity::Error,
            code: None,
            title: String::new(),
        }
    }
}

#[derive(Debug)]
pub enum ReportKind {
    Formatter(String, FormatterReportFileDetail),
    Error(String, ReportErrorKind),
}

impl Report {
    /// Creates or updates a stat
    pub fn push_detail_report(&mut self, stat: ReportKind) {
        match stat {
            ReportKind::Formatter(path, stat) => {
                self.formatter.insert_file_content(path, stat);
            }
            ReportKind::Error(path, error) => {
                self.diagnostics.insert(path, error);
            }
        }
    }

    /// It tracks a generic diagnostic
    pub fn push_error(&mut self, path: String, err: ReportErrorKind) {
        self.diagnostics.insert(path, err);
    }

    pub fn set_formatter_summary(&mut self, summary: FormatterReportSummary) {
        self.formatter.set_summary(summary);
    }

    pub fn as_serialized_reports(&self) -> Result<String, RomeError> {
        serde_json::to_string(&self).map_err(|_| RomeError::NotFound)
    }
}
