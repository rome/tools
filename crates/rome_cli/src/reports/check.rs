use crate::{Execution, TraversalMode};
use indexmap::IndexMap;
use rome_console::{markup, Console, ConsoleExt};
use serde::Serialize;
use std::time::Duration;

#[derive(Debug, Default, Serialize)]
pub struct LinterReport {
    summary: LinterReportSummary,

    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    files: IndexMap<String, LinterReportFileDetail>,
}

impl LinterReport {
    pub fn set_summary(&mut self, summary: LinterReportSummary) {
        self.summary = summary;
    }

    pub(crate) fn summary(&self) -> &LinterReportSummary {
        &self.summary
    }

    pub(crate) fn count(&self) -> &usize {
        &self.summary.count
    }

    pub(crate) fn skipped(&self) -> &usize {
        &self.summary.skipped
    }

    pub(crate) fn insert_file_content(&mut self, path: String, details: LinterReportFileDetail) {
        self.files.insert(path, details);
    }

    pub(crate) fn report_to_console(
        &self,
        execution: &Execution,
        console: &mut dyn Console,
        duration: &Duration,
    ) {
        if let TraversalMode::Check { .. } = execution.traversal_mode() {
            if execution.as_fix_file_mode().is_some() {
                console.log(markup! {
                    <Info>"Fixed "{self.count()}" files in "{duration}</Info>
                });
            } else {
                console.log(markup! {
                    <Info>"Checked "{self.count()}" files in "{duration}</Info>
                });
            }

            if self.skipped() > &0_usize {
                console.log(markup! {
                    <Warn>"Skipped "{self.skipped()}" suggested fixes.\n"</Warn>
                    <Info>"If you wish to apply the suggested fixes, use the command "<Emphasis>"rome check --apply-suggested\n"</Emphasis></Info>
                })
            }
        }
    }

    pub(crate) fn report_to_json(&self, console: &mut dyn Console) {
        todo!()
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinterReportSummary {
    /// how many files were  linted
    pub(crate) count: usize,

    pub(crate) skipped: usize,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinterReportFileDetail {
    pub(crate) rule_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) safe_fix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) suggested_fix: Option<String>,
}
