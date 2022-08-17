use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatterStats {
    /// Useful information of the execution
    summary: Option<FormatterStatSummary>,

    /// The key is the path of the file
    details: HashMap<String, FormatterStatDetail>,
}

impl FormatterStats {
    pub(crate) fn insert_stat_detail(&mut self, stat: String, detail: FormatterStatDetail) {
        self.details.insert(stat, detail);
    }

    pub(crate) fn set_summary(&mut self, summary: FormatterStatSummary) {
        self.summary = Some(summary);
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatterStatSummary {
    /// how many files were compared
    files_compared: Option<usize>,
    /// how many files were written
    files_written: Option<usize>,
}

impl FormatterStatSummary {
    pub(crate) fn set_files_compared(&mut self, files_compared: usize) {
        self.files_compared = Some(files_compared)
    }

    pub(crate) fn self_files_written(&mut self, files_written: usize) {
        self.files_written = Some(files_written)
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatterStatDetail {
    /// The new content emitted by the formatter
    pub formatted_content: Option<String>,
}
