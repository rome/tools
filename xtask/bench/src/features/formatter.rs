use crate::BenchmarkSummary;
use rome_formatter::{format, FormatOptions};
use rslint_parser::{parse, Syntax};
use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct FormatterMeasurement {
    id: String,
    formatting: Duration,
}
pub fn benchmark_format_lib(id: &str, code: &str) -> BenchmarkSummary {
    let formatter_timer = timing::start();
    run_format(code);
    let formatter_duration = formatter_timer.stop();

    BenchmarkSummary::Formatter(FormatterMeasurement {
        id: id.to_string(),
        formatting: formatter_duration,
    })
}

pub fn run_format(code: &str) {
    let syntax = Syntax::default().module();
    let root = parse(code, 0, syntax).syntax();

    let _ = format(FormatOptions::default(), &root);
}

impl FormatterMeasurement {
    fn total(&self) -> Duration {
        self.formatting
    }

    pub(crate) fn summary(&self) -> String {
        format!("{}, Formatting: {:?}", self.id, self.total(),)
    }
}

impl Display for FormatterMeasurement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "\tFormatting: {:>10?}", self.formatting);
        let _ = writeln!(f, "\t              ----------");
        let _ = writeln!(f, "\tTotal:        {:>10?}", self.total());

        Ok(())
    }
}
