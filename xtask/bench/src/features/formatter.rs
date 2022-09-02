#[cfg(feature = "dhat-on")]
use crate::features::print_diff;
use crate::BenchmarkSummary;
use rome_formatter::Printed;
use rome_js_formatter::context::JsFormatOptions;
use rome_js_formatter::format_node;
use rome_js_syntax::{JsSyntaxNode, SourceType};
use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct FormatterMeasurement {
    id: String,
    formatting: Duration,
}
pub fn benchmark_format_lib(
    id: &str,
    root: &JsSyntaxNode,
    source_type: SourceType,
) -> BenchmarkSummary {
    let formatter_timer = timing::start();
    run_format(root, source_type);
    let formatter_duration = formatter_timer.stop();

    BenchmarkSummary::Formatter(FormatterMeasurement {
        id: id.to_string(),
        formatting: formatter_duration,
    })
}

pub fn run_format(root: &JsSyntaxNode, source_type: SourceType) -> Printed {
    #[cfg(feature = "dhat-on")]
    let stats = {
        println!("Start");
        dhat::get_stats().unwrap()
    };

    let formatted = format_node(JsFormatOptions::new(source_type), root).unwrap();

    #[cfg(feature = "dhat-on")]
    let stats = {
        println!("Formatted");
        print_diff(stats, dhat::get_stats().unwrap())
    };

    let printed = formatted.print();
    drop(formatted);

    #[cfg(feature = "dhat-on")]
    {
        println!("Printed");
        print_diff(stats, dhat::get_stats().unwrap());
    }

    #[allow(clippy::let_and_return)]
    printed
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
