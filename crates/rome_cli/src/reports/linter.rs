use crate::Execution;
use rome_console::{Console, markup};

#[derive(Debug, Default)]
pub struct LinterReport {
    summary: LinterReportSummary,
}

impl LinterReport {
    pub fn set_summary(&mut self, summary: LinterReportSummary) {
        self.summary = summary;
    }

    pub fn print_summary(&self, execution: &Execution, console: &mut dyn Console) {
        if execution.as_fix_file_mode().is_some() {
            console.log(markup! {
            <Info>"Fixed "{self.count()}" files in "{duration}</Info>
            });
        } else {
            console.log(markup! {
            <Info>"Checked "{self.count()}" files in "{duration}</Info>
            });
        }
    }
}
#[derive(Debug, Default)]
pub struct LinterReportSummary {}
