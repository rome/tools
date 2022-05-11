use crate::BenchmarkSummary;
use rome_formatter::Printed;
use rome_js_formatter::format_node;
use rome_js_formatter::options::JsFormatOptions;
use rome_js_syntax::JsSyntaxNode;
use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct FormatterMeasurement {
    id: String,
    formatting: Duration,
}
pub fn benchmark_format_lib(id: &str, root: &JsSyntaxNode) -> BenchmarkSummary {
    let formatter_timer = timing::start();
    run_format(root);
    let formatter_duration = formatter_timer.stop();

    BenchmarkSummary::Formatter(FormatterMeasurement {
        id: id.to_string(),
        formatting: formatter_duration,
    })
}

pub fn run_format(root: &JsSyntaxNode) -> Printed {
    format_node(JsFormatOptions::default(), root)
        .unwrap()
        .print()
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
