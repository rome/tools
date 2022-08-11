use std::env;

use rome_cli::Termination;
use rome_lsp::ServerFactory;
use tokio::runtime::Runtime;
use tracing::{debug_span, Instrument};
use tracing_subscriber::{prelude::*, registry, EnvFilter, Layer};
use tracing_tree::HierarchicalLayer;

use crate::service::{print_socket, run_daemon};

pub fn run_server_session() -> Result<(), Termination> {
    setup_tracing_subscriber();

    let rt = Runtime::new()?;
    let factory = ServerFactory::default();
    let span = debug_span!("Running LSP Server", pid = std::process::id());
    rt.block_on(run_daemon(factory).instrument(span))?;

    Ok(())
}

pub fn print_server_socket() -> Result<(), Termination> {
    let rt = Runtime::new()?;
    rt.block_on(print_socket())?;
    Ok(())
}

fn setup_tracing_subscriber() {
    /// This filter enables:
    /// - All spans and events at level info or higher
    /// - All spans and events in the `rome_lsp` and `rome_js_parser` crates
    const LOGGING_FILTER: &str = "info,rome_lsp=trace,rome_js_parser=trace";

    let logs_dir = env::temp_dir().join("rome-logs");
    let file_appender = tracing_appender::rolling::hourly(logs_dir, "server.log");

    registry()
        .with(
            HierarchicalLayer::default()
                .with_indent_lines(true)
                .with_indent_amount(2)
                .with_bracketed_fields(true)
                .with_targets(true)
                .with_ansi(false)
                .with_writer(file_appender)
                .with_filter(EnvFilter::new(LOGGING_FILTER)),
        )
        .init();
}
