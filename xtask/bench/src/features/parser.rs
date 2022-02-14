use crate::BenchmarkSummary;
use itertools::Itertools;
use rslint_errors::file::SimpleFile;
use rslint_errors::{Diagnostic, Emitter, Severity};
use rslint_parser::ast::JsAnyRoot;
use rslint_parser::{Parse, Syntax};
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ParseMeasurement {
    id: String,
    code: String,
    tokenization: Duration,
    parsing: Duration,
    tree_sink: Duration,
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
pub fn benchmark_parse_lib(id: &str, code: &str, syntax: Syntax) -> BenchmarkSummary {
    #[cfg(feature = "dhat-on")]
    println!("Start");
    #[cfg(feature = "dhat-on")]
    let stats = dhat::get_stats().unwrap();

    let tokenizer_timer = timing::start();
    let (tokens, mut diagnostics) = rslint_parser::tokenize(code, 0);
    let tok_source = rslint_parser::TokenSource::new(code, &tokens);
    let tokenization_duration = tokenizer_timer.stop();

    #[cfg(feature = "dhat-on")]
    println!("Tokenizer");
    #[cfg(feature = "dhat-on")]
    let stats = print_diff(stats, dhat::get_stats().unwrap());

    let parser_timer = timing::start();
    let (events, parsing_diags, tokens) = {
        let mut parser = rslint_parser::Parser::new(tok_source, 0, syntax);
        rslint_parser::syntax::program::parse(&mut parser);
        let (events, parsing_diags) = parser.finish();
        (events, parsing_diags, tokens)
    };
    let parse_duration = parser_timer.stop();

    #[cfg(feature = "dhat-on")]
    println!("Parsed");
    #[cfg(feature = "dhat-on")]
    let stats = print_diff(stats, dhat::get_stats().unwrap());

    let tree_sink_timer = timing::start();
    let mut tree_sink = rslint_parser::LosslessTreeSink::new(code, &tokens);
    rslint_parser::process(&mut tree_sink, events, parsing_diags);
    let (_green, sink_diags) = tree_sink.finish();
    let tree_sink_duration = tree_sink_timer.stop();

    #[cfg(feature = "dhat-on")]
    println!("Tree-Sink");
    #[cfg(feature = "dhat-on")]
    print_diff(stats, dhat::get_stats().unwrap());

    diagnostics.extend(sink_diags);

    BenchmarkSummary::Parser(ParseMeasurement {
        id: id.to_string(),
        code: code.to_string(),
        tokenization: tokenization_duration,
        parsing: parse_duration,
        tree_sink: tree_sink_duration,
        diagnostics,
    })
}

pub fn run_parse(code: &str, syntax: Syntax) -> Parse<JsAnyRoot> {
    rslint_parser::parse(code, 0, syntax)
}

impl ParseMeasurement {
    fn total(&self) -> Duration {
        self.tokenization.add(self.parsing).add(self.tree_sink)
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "{}, Total Time: {:?}, tokenization: {:?}, parsing: {:?}, tree_sink: {:?}",
            self.id,
            self.total(),
            self.tokenization,
            self.parsing,
            self.tree_sink,
        )
    }
}

impl Display for ParseMeasurement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "\tTokenization: {:>10?}", self.tokenization);
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
