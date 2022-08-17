use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatterReport {
    /// Useful information of the execution
    summary: Option<FormatterReportSummary>,

    /// The key is the path of the file
    files: HashMap<String, FormatterReportFileDetail>,
}

impl FormatterReport {
    pub(crate) fn insert_file_content(&mut self, path: String, detail: FormatterReportFileDetail) {
        self.files.insert(path, detail);
    }

    pub(crate) fn set_summary(&mut self, summary: FormatterReportSummary) {
        self.summary = Some(summary);
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatterReportSummary {
    /// how many files were compared
    files_compared: Option<usize>,
    /// how many files were written
    files_written: Option<usize>,
}

impl FormatterReportSummary {
    pub(crate) fn set_files_compared(&mut self, files_compared: usize) {
        self.files_compared = Some(files_compared)
    }

    pub(crate) fn self_files_written(&mut self, files_written: usize) {
        self.files_written = Some(files_written)
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatterReportFileDetail {
    /// The new content emitted by the formatter
    pub formatted_content: Option<String>,
}
