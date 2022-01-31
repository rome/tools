use crate::BenchSummary;
use rome_formatter::{format, FormatOptions};
use rslint_parser::{parse, Syntax};
use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BenchmarkFormatterResult {
    id: String,
    formatting: Duration,
}
pub fn benchmark_format_lib(id: &str, code: &str) -> BenchSummary {
    let syntax = Syntax::default().module();
    let root = parse(code, 0, syntax).syntax();

    let formatter_timer = timing::start();
    let _ = format(FormatOptions::default(), &root);
    let formatter_duration = formatter_timer.stop();

    BenchSummary::Formatter(BenchmarkFormatterResult {
        id: id.to_string(),
        formatting: formatter_duration,
    })
}

impl BenchmarkFormatterResult {
    fn total(&self) -> Duration {
        self.formatting
    }

    pub(crate) fn summary(&self) -> String {
        format!("{}, Formatting: {:?}", self.id, self.total(),)
    }
}

impl Display for BenchmarkFormatterResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "\tFormatting: {:>10?}", self.formatting);
        let _ = writeln!(f, "\t              ----------");
        let _ = writeln!(f, "\tTotal:        {:>10?}", self.total());

        // TODO: add diagnostics once the formatter API is able to return them from a formatter error
        Ok(())
    }
}
