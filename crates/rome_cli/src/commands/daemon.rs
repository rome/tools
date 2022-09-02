use std::env;

use rome_lsp::ServerFactory;
use rome_service::workspace::WorkspaceClient;
use tokio::runtime::Runtime;
use tracing::{debug_span, metadata::LevelFilter, Instrument, Metadata};
use tracing_subscriber::{
    layer::{Context, Filter},
    prelude::*,
    registry, Layer,
};
use tracing_tree::HierarchicalLayer;

use crate::{
    open_transport,
    service::{self, ensure_daemon, run_daemon},
    Termination,
};

pub(crate) fn start() -> Result<(), Termination> {
    let rt = Runtime::new()?;
    rt.block_on(ensure_daemon())?;
    Ok(())
}

pub(crate) fn stop() -> Result<(), Termination> {
    let rt = Runtime::new()?;

    if let Some(transport) = open_transport(rt)? {
        let client = WorkspaceClient::new(transport)?;
        // This can be an error if the connection is closed before the empty response is sent
        client.shutdown().ok();
    }

    Ok(())
}

pub(crate) fn run_server() -> Result<(), Termination> {
    setup_tracing_subscriber();

    let rt = Runtime::new()?;
    let factory = ServerFactory::default();
    let cancellation = factory.cancellation();
    let span = debug_span!("Running Server", pid = std::process::id());

    rt.block_on(async move {
        tokio::select! {
            res = run_daemon(factory).instrument(span) => {
                match res {
                    Ok(never) => match never {},
                    Err(err) => Err(err.into()),
                }
            }
            _ = cancellation.notified() => {
                tracing::info!("Received shutdown signal");
                Ok(())
            }
        }
    })
}

pub(crate) fn print_socket() -> Result<(), Termination> {
    let rt = Runtime::new()?;
    rt.block_on(service::print_socket())?;
    Ok(())
}

/// Setup the [tracing]-based logging system for the server
/// The events received by the subscriber are filtered at the `info` level,
/// then printed using the [HierarchicalLayer] layer, and the resulting text
/// is written to log files rotated on a hourly basis (in
/// `rome-logs/server.log.yyyy-MM-dd-HH` files inside the system temporary
/// directory)
fn setup_tracing_subscriber() {
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
                .with_filter(LoggingFilter),
        )
        .init();
}

/// Tracing filter enabling:
/// - All spans and events at level info or higher
/// - All spans and events at level debug in crates whose name starts with `rome`
struct LoggingFilter;

impl<S> Filter<S> for LoggingFilter {
    fn enabled(&self, meta: &Metadata<'_>, _cx: &Context<'_, S>) -> bool {
        let filter = if meta.target().starts_with("rome") {
            LevelFilter::DEBUG
        } else {
            LevelFilter::INFO
        };

        meta.level() <= &filter
    }

    fn max_level_hint(&self) -> Option<LevelFilter> {
        Some(LevelFilter::DEBUG)
    }
}
