use crate::utils::print_stats_diff;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BenchmarkParseResult {
    id: String,
    formatting: Duration,
}
pub fn benchmar_format_lib(id: &str, code: &str) -> BenchmarkParseResult {
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
    let stats = print_stats_diff(stats, dhat::get_stats().unwrap());

    let parser_timer = timing::start();
    let (events, parsing_diags, tokens) = {
        let mut parser =
            rslint_parser::Parser::new(tok_source, 0, rslint_parser::Syntax::default().script());
        rslint_parser::syntax::program::parse(&mut parser);
        let (events, parsing_diags) = parser.finish();
        (events, parsing_diags, tokens)
    };
    let parse_duration = parser_timer.stop();

    #[cfg(feature = "dhat-on")]
    println!("Parsed");
    #[cfg(feature = "dhat-on")]
    let stats = print_stats_diff(stats, dhat::get_stats().unwrap());

    let tree_sink_timer = timing::start();
    let mut tree_sink = rslint_parser::LosslessTreeSink::new(code, &tokens);
    rslint_parser::process(&mut tree_sink, events, parsing_diags);
    let (_green, sink_diags) = tree_sink.finish();
    let tree_sink_duration = tree_sink_timer.stop();

    #[cfg(feature = "dhat-on")]
    println!("Tree-Sink");
    #[cfg(feature = "dhat-on")]
    print_stats_diff(stats, dhat::get_stats().unwrap());

    diagnostics.extend(sink_diags);
    BenchmarkParseResult { id: id.to_string() }
}

impl Display for BenchmarkParseResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // let _ = writeln!(f, "\tTokenization: {:>10?}", self.tokenization);
        // let _ = writeln!(f, "\tParsing:      {:>10?}", self.parsing);
        // let _ = writeln!(f, "\tTree_sink:    {:>10?}", self.tree_sink);
        // let _ = writeln!(f, "\t              ----------");
        // let _ = writeln!(f, "\tTotal:        {:>10?}", self.total());
        //
        // let _ = writeln!(f, "\tDiagnostics");
        // let diagnostics = &self.diagnostics.iter().group_by(|x| x.severity);
        // for (severity, items) in diagnostics {
        //     let _ = writeln!(f, "\t\t{:?}: {}", severity, items.count());
        // }

        Ok(())
    }
}
