use tracing::{debug_span, Instrument};
use tracing_subscriber::{layer::SubscriberExt, prelude::*, registry, EnvFilter};
use tracing_tree::HierarchicalLayer;

use rome_lsp::server::run_server;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// This filter enables:
/// - All spans and events at level info or higher
/// - All spans and events in the `rome_lsp` and `rome_js_parser` crates
const LOGGING_FILTER: &str = "info,rome_lsp=trace,rome_js_parser=trace";

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    registry()
        .with(
            HierarchicalLayer::default()
                .with_indent_lines(true)
                .with_indent_amount(2)
                .with_bracketed_fields(true)
                .with_targets(true)
                .with_filter(EnvFilter::new(LOGGING_FILTER)),
        )
        .init();

    let span = debug_span!("Running LSP Server", pid = std::process::id());
    run_server(stdin, stdout).instrument(span).await
}
