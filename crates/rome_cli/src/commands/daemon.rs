use std::env;

use rome_lsp::ServerFactory;
use rome_service::workspace::WorkspaceClient;
use tokio::runtime::Runtime;
use tracing::{debug_span, Instrument};
use tracing_subscriber::{prelude::*, registry, EnvFilter, Layer};
use tracing_tree::HierarchicalLayer;

use crate::{
    open_transport,
    service::{ensure_daemon, print_socket, run_daemon},
    CliSession, Termination,
};

/// Handler for the "daemon" command of the Rome CLI
pub(crate) fn daemon(mut session: CliSession) -> Result<(), Termination> {
    let subcommand = session
        .args
        .subcommand()
        .map_err(|source| Termination::ParseError {
            argument: "daemon <command>",
            source,
        })?;

    match subcommand.as_deref() {
        Some("start") => start_server(),
        Some("stop") => stop_server(),

        // Internal private commands
        Some("__run_server") => run_server_session(),
        Some("__print_socket") => print_server_socket(),

        // Print the general help if no subcommand was specified / the subcommand is `help`
        None | Some("help") => crate::commands::help::help(session, Some("daemon")),

        Some(cmd) => Err(Termination::UnknownCommand {
            command: format!("daemon {cmd}"),
        }),
    }
}

fn start_server() -> Result<(), Termination> {
    let rt = Runtime::new()?;
    rt.block_on(ensure_daemon())?;
    Ok(())
}

fn stop_server() -> Result<(), Termination> {
    let rt = Runtime::new()?;

    if let Some(transport) = open_transport(rt)? {
        let client = WorkspaceClient::new(transport)?;
        // This can be an error if the connection is closed before the empty response is sent
        client.shutdown().ok();
    }

    Ok(())
}

fn run_server_session() -> Result<(), Termination> {
    setup_tracing_subscriber();

    let rt = Runtime::new()?;
    let factory = ServerFactory::default();
    let cancellation = factory.cancellation();
    let span = debug_span!("Running Server", pid = std::process::id());

    rt.block_on(async move {
        tokio::select! {
            res = run_daemon(factory).instrument(span) => {
                match res {
                    Ok(_) => unreachable!(),
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

fn print_server_socket() -> Result<(), Termination> {
    let rt = Runtime::new()?;
    rt.block_on(print_socket())?;
    Ok(())
}

/// Setup the [tracing]-based logging system for the server
/// The events received by the subscriber are filtered at the `info` level,
/// then printed using the [HierarchicalLayer] layer, and the resulting text
/// is written to log files rotated on a hourly basis (in
/// `rome-logs/server.log.yyyy-MM-dd-HH` files inside the system temporary
/// directory)
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
