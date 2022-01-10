use tracing::{debug_span, Instrument};
use tracing_subscriber::{layer::SubscriberExt, Registry};
use tracing_tree::HierarchicalLayer;

use rome_lsp::server::run_server;

#[tokio::main]
async fn main() {
	let stdin = tokio::io::stdin();
	let stdout = tokio::io::stdout();

	let layer = HierarchicalLayer::default()
		.with_indent_lines(true)
		.with_indent_amount(2)
		.with_bracketed_fields(true);

	let subscriber = Registry::default().with(layer);
	tracing::subscriber::set_global_default(subscriber).unwrap();

	let span = debug_span!("Running LSP Server", pid = std::process::id());
	run_server(stdin, stdout).instrument(span).await
}
