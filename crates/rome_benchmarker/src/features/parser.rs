use crate::BenchmarkSummary;
use itertools::Itertools;
use rome_js_syntax::JsAnyRoot;
use rslint_errors::file::SimpleFile;
use rslint_errors::{Diagnostic, Emitter, Severity};
use rslint_parser::{Parse, SourceType};
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub struct ParseMeasurement {
    id: String,
    #[serde(skip_serializing)]
    code: String,
    parsing: Duration,
    tree_sink: Duration,
    #[serde(skip_serializing)]
    diagnostics: Vec<Diagnostic>,
}

#[cfg(feature = "dhat-on")]
fn print_diff(before: dhat::Stats, current: dhat::Stats) -> dhat::Stats {
    use humansize::{file_size_opts as options, FileSize};

    println!("\tMemory");
    if let Some(heap) = &current.heap {
        println!("\t\tCurrent Blocks: {}", heap.curr_blocks);
        println!(
            "\t\tCurrent Bytes: {}",
            heap.curr_bytes.file_size(options::CONVENTIONAL).unwrap()
        );
        println!("\t\tMax Blocks: {}", heap.max_blocks);
        println!(
            "\t\tMax Bytes: {}",
            heap.max_bytes.file_size(options::CONVENTIONAL).unwrap()
        );
    }

    println!(
        "\t\tTotal Blocks: {}",
        current.total_blocks - before.total_blocks
    );
    println!(
        "\t\tTotal Bytes: {}",
        (current.total_bytes - before.total_bytes)
            .file_size(options::CONVENTIONAL)
            .unwrap()
    );

    current
}
pub fn benchmark_parse_lib(id: &str, code: &str, source_type: SourceType) -> BenchmarkSummary {
    #[cfg(feature = "dhat-on")]
    println!("Start");
    #[cfg(feature = "dhat-on")]
    let stats = dhat::get_stats().unwrap();

    let parser_timer = timing::start();

    let (events, diagnostics, tokens) = {
        let mut parser = rslint_parser::Parser::new(code, 0, source_type);
        rslint_parser::syntax::program::parse(&mut parser);
        let (events, tokens, diagnostics) = parser.finish();
        (events, diagnostics, tokens)
    };
    let parse_duration = parser_timer.stop();

    #[cfg(feature = "dhat-on")]
    println!("Parsed");
    #[cfg(feature = "dhat-on")]
    let stats = print_diff(stats, dhat::get_stats().unwrap());

    let tree_sink_timer = timing::start();
    let mut tree_sink = rslint_parser::LosslessTreeSink::new(code, &tokens);
    rslint_parser::process(&mut tree_sink, events, diagnostics);
    let (_green, diagnostics) = tree_sink.finish();
    let tree_sink_duration = tree_sink_timer.stop();

    #[cfg(feature = "dhat-on")]
    println!("Tree-Sink");
    #[cfg(feature = "dhat-on")]
    print_diff(stats, dhat::get_stats().unwrap());

    BenchmarkSummary::Parser(ParseMeasurement {
        id: id.to_string(),
        code: code.to_string(),
        parsing: parse_duration,
        tree_sink: tree_sink_duration,
        diagnostics,
    })
}

pub fn run_parse(code: &str, source_type: SourceType) -> Parse<JsAnyRoot> {
    rslint_parser::parse(code, 0, source_type)
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
