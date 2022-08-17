pub mod formatter;

use crate::stats::formatter::FormatterStatDetail;
use crate::FormatterStatSummary;
use formatter::FormatterStats;
use rome_console::codespan::Severity;
use rome_service::RomeError;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Default, Serialize)]
pub struct Stats {
    /// Information relative to the formatter
    formatter: FormatterStats,

    /// Diagnostics tracked during a generic traversal
    diagnostics: HashMap<String, StatErrorKind>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum StatErrorKind {
    Diagnostic(StatDiagnostic),
    Diff(StatDiff),
}

#[derive(Debug, Serialize)]
pub struct StatDiagnostic {
    pub severity: Severity,
    pub code: Option<String>,
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct StatDiff {
    pub severity: Severity,
    pub before: String,
    pub after: String,
}

impl Default for StatDiagnostic {
    fn default() -> Self {
        Self {
            severity: Severity::Error,
            code: None,
            title: String::new(),
        }
    }
}

#[derive(Debug)]
pub enum StatKind {
    Formatter(String, FormatterStatDetail),
    Error(String, StatErrorKind),
}

impl Stats {
    /// Creates or updates a stat
    pub fn push_detail_stat(&mut self, stat: StatKind) {
        match stat {
            StatKind::Formatter(path, stat) => {
                self.formatter.insert_stat_detail(path, stat);
            }
            StatKind::Error(path, error) => {
                self.diagnostics.insert(path, error);
            }
        }
    }

    /// It tracks a generic diagnostic
    pub fn push_error(&mut self, path: String, err: StatErrorKind) {
        self.diagnostics.insert(path, err);
    }

    pub fn set_formatter_summary(&mut self, summary: FormatterStatSummary) {
        self.formatter.set_summary(summary);
    }

    pub fn as_serialized_stats(&self) -> Result<String, RomeError> {
        serde_json::to_string(&self).map_err(|_| RomeError::NotFound)
    }
}
