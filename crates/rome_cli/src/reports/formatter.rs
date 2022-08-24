use crate::{Execution, TraversalMode};
use indexmap::IndexMap;
use rome_console::{markup, Console, ConsoleExt};
use rome_service::{ConfigurationError, RomeError};
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatterReport {
    /// Useful information of the execution
    summary: FormatterReportSummary,

    /// The key is the path of the file
    files: IndexMap<String, FormatterReportFileDetail>,
}

impl FormatterReport {
    pub(crate) fn insert_file_content(&mut self, path: String, detail: FormatterReportFileDetail) {
        self.files.insert(path, detail);
    }

    pub(crate) fn set_summary(&mut self, summary: FormatterReportSummary) {
        self.summary = summary;
    }

    pub(crate) fn summary(&self) -> &FormatterReportSummary {
        &self.summary
    }

    pub(crate) fn formatted(&self) -> &usize {
        &self.summary.formatted
    }

    pub(crate) fn report_to_console(
        &self,
        execution: &Execution,
        console: &mut dyn Console,
        duration: &Duration,
    ) {
        if let TraversalMode::Format { write, .. } = execution.traversal_mode() {
            if *write {
                console.log(markup! {
                    <Info>"Formatted "{self.formatted()}" files in "{duration}</Info>
                });
            } else {
                console.log(markup! {
                    <Info>"Compared "{self.formatted()}" files in "{duration}</Info>
                });
            }
        }
    }

    pub(crate) fn report_to_json(&self) -> Result<String, RomeError> {
        let serialized = serde_json::to_string(&self)
            .map_err(|_| RomeError::Configuration(ConfigurationError::SerializationError))?;

        Ok(serialized)
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatterReportSummary {
    /// how many files were compared
    pub(crate) formatted: usize,
}

impl FormatterReportSummary {
    pub(crate) fn set_count(&mut self, count: usize) {
        self.formatted = count
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatterReportFileDetail {
    /// The new content emitted by the formatter
    pub formatted_content: Option<String>,
}
