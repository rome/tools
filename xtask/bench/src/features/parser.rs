#[cfg(feature = "dhat-heap")]
use crate::features::print_diff;
use crate::BenchmarkSummary;
use itertools::Itertools;
use rome_diagnostics::file::SimpleFile;
use rome_diagnostics::{Diagnostic, Emitter, Severity};
use rome_js_parser::{parse_common, Parse};
use rome_js_syntax::{JsAnyRoot, SourceType};
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ParseMeasurement {
    id: String,
    code: String,
    parsing: Duration,
    tree_sink: Duration,
    diagnostics: Vec<Diagnostic>,
}

pub fn benchmark_parse_lib(id: &str, code: &str, source_type: SourceType) -> BenchmarkSummary {
    #[cfg(feature = "dhat-heap")]
    println!("Start");
    #[cfg(feature = "dhat-heap")]
    let stats = dhat::HeapStats::get();

    let parser_timer = timing::start();
    let (events, diagnostics, trivia) = parse_common(code, 0, source_type);
    let parse_duration = parser_timer.stop();

    #[cfg(feature = "dhat-heap")]
    println!("Parsed");
    #[cfg(feature = "dhat-heap")]
    let stats = print_diff(stats, dhat::HeapStats::get());

    let tree_sink_timer = timing::start();
    let mut tree_sink = rome_js_parser::LosslessTreeSink::new(code, &trivia);
    rome_js_parser::process(&mut tree_sink, events, diagnostics);
    let (_green, diagnostics) = tree_sink.finish();
    let tree_sink_duration = tree_sink_timer.stop();

    #[cfg(feature = "dhat-heap")]
    println!("Tree-Sink");
    #[cfg(feature = "dhat-heap")]
    print_diff(stats, dhat::HeapStats::get());

    BenchmarkSummary::Parser(ParseMeasurement {
        id: id.to_string(),
        code: code.to_string(),
        parsing: parse_duration,
        tree_sink: tree_sink_duration,
        diagnostics,
    })
}

pub fn run_parse(code: &str, source_type: SourceType) -> Parse<JsAnyRoot> {
    rome_js_parser::parse(code, 0, source_type)
}

impl ParseMeasurement {
    fn total(&self) -> Duration {
        self.parsing.add(self.tree_sink)
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "{}, Total Time: {:?}, parsing: {:?}, tree_sink: {:?}",
            self.id,
            self.total(),
            self.parsing,
            self.tree_sink,
        )
    }
}

impl Display for ParseMeasurement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "\tParsing:      {:>10?}", self.parsing);
        let _ = writeln!(f, "\tTree_sink:    {:>10?}", self.tree_sink);
        let _ = writeln!(f, "\t              ----------");
        let _ = writeln!(f, "\tTotal:        {:>10?}", self.total());

        let _ = writeln!(f, "\tDiagnostics");

        let diagnostics = &self.diagnostics.iter().group_by(|x| x.severity);
        for (severity, items) in diagnostics {
            let _ = writeln!(f, "\t\t{:?}: {}", severity, items.count());
        }

        let file = SimpleFile::new(self.id.to_string(), self.code.clone());
        let mut emitter = Emitter::new(&file);

        for diagnostic in self
            .diagnostics
            .iter()
            .filter(|diag| diag.severity == Severity::Error)
        {
            emitter.emit_stderr(diagnostic, true).unwrap();
        }

        Ok(())
    }
}
