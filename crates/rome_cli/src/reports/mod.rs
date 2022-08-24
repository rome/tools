pub(crate) mod check;
pub mod formatter;
pub mod reporter;

pub use crate::reports::formatter::{FormatterReportFileDetail, FormatterReportSummary};
pub use crate::reports::reporter::{ReportDiagnostic, ReportDiagnosticKind};
use formatter::FormatterReport;
use rome_service::RomeError;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Default, Serialize)]
pub struct Report {
    /// Information relative to the formatter
    formatter: FormatterReport,

    /// Diagnostics tracked during a generic traversal
    ///
    /// The key is the path of the file where the diagnostics occurred
    diagnostics: HashMap<String, ReportDiagnosticKind>,
}

pub enum ReportKind {
    Formatter(String, FormatterReportFileDetail),
    Error(String, ReportDiagnosticKind),
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
    pub fn push_error(&mut self, path: String, err: ReportDiagnosticKind) {
        self.diagnostics.insert(path, err);
    }

    pub fn set_formatter_summary(&mut self, summary: FormatterReportSummary) {
        self.formatter.set_summary(summary);
    }

    pub fn as_serialized_reports(&self) -> Result<String, RomeError> {
        serde_json::to_string(&self).map_err(|_| RomeError::NotFound)
    }
}
