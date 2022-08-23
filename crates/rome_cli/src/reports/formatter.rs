use crate::{Execution, TraversalMode};
use rome_console::{markup, Console, ConsoleExt};
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatterReport {
    /// Useful information of the execution
    pub(crate) summary: FormatterReportSummary,

    /// The key is the path of the file
    pub(crate) files: HashMap<String, FormatterReportFileDetail>,
}

impl FormatterReport {
    pub(crate) fn insert_file_content(&mut self, path: String, detail: FormatterReportFileDetail) {
        self.files.insert(path, detail);
    }

    pub(crate) fn set_summary(&mut self, summary: FormatterReportSummary) {
        self.summary = summary;
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatterReportSummary {
    /// how many files were compared
    pub(crate) count: usize,
}

impl FormatterReportSummary {
    pub(crate) fn set_count(&mut self, count: usize) {
        self.count = count
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
                    <Info>"Formatted "{self.count}" files in "{duration}</Info>
                });
            } else {
                console.log(markup! {
                    <Info>"Compared "{self.count}" files in "{duration}</Info>
                });
            }
        }
    }

    pub(crate) fn report_to_json(&self, console: &mut dyn Console) {}
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatterReportFileDetail {
    /// The new content emitted by the formatter
    pub formatted_content: Option<String>,
}
