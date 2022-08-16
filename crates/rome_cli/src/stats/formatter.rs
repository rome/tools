use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

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
    /// how long it took to run the formatter
    duration: Option<Duration>,
    /// how many files were compared
    files_compared: Option<usize>,
    /// how many files were written
    files_written: Option<usize>,
}

impl FormatterStatSummary {
    pub(crate) fn set_duration(&mut self, duration: Duration) {
        self.duration = Some(duration);
    }

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
    pub new_content: Option<String>,
}
