use rome_console::{markup, ConsoleExt};
use rome_lsp::ServerFactory;
use rome_service::{workspace::WorkspaceClient, TransportError, WorkspaceError};
use std::{env, fs, path::PathBuf};
use tokio::io;
use tokio::runtime::Runtime;
use tracing::subscriber::Interest;
use tracing::{debug_span, metadata::LevelFilter, Instrument, Metadata};
use tracing_subscriber::{
    layer::{Context, Filter},
    prelude::*,
    registry, Layer,
};
use tracing_tree::HierarchicalLayer;

use crate::{
    open_transport,
    service::{self, ensure_daemon, open_socket, run_daemon},
    CliDiagnostic, CliSession,
};

pub(crate) fn start(session: CliSession) -> Result<(), CliDiagnostic> {
    let rt = Runtime::new()?;
    let did_spawn = rt.block_on(ensure_daemon(false))?;

    if did_spawn {
        session.app.console.log(markup! {
            "The Rome server was successfully started"
        });
    } else {
        session.app.console.log(markup! {
            "The Rome server was already running"
        });
    }

    Ok(())
}

pub(crate) fn stop(session: CliSession) -> Result<(), CliDiagnostic> {
    let rt = Runtime::new()?;

    if let Some(transport) = open_transport(rt)? {
        let client = WorkspaceClient::new(transport)?;
        match client.shutdown() {
            // The `ChannelClosed` error is expected since the server can
            // shutdown before sending a response
            Ok(()) | Err(WorkspaceError::TransportError(TransportError::ChannelClosed)) => {}
            Err(err) => return Err(CliDiagnostic::from(err)),
        };

        session.app.console.log(markup! {
            "The Rome server was successfully stopped"
        });
    } else {
        session.app.console.log(markup! {
            "The Rome server was not running"
        });
    }

    Ok(())
}

pub(crate) fn print_socket() -> Result<(), CliDiagnostic> {
    let rt = Runtime::new()?;
    rt.block_on(service::print_socket())?;
    Ok(())
}

pub(crate) fn lsp_proxy() -> Result<(), CliDiagnostic> {
    let rt = Runtime::new()?;
    rt.block_on(start_lsp_proxy(&rt))?;

    Ok(())
}

/// Start a proxy process.
/// Receives a process via `stdin` and then copy the content to the LSP socket.
/// Copy to the process on `stdout` when the LSP responds to a message
async fn start_lsp_proxy(rt: &Runtime) -> Result<(), CliDiagnostic> {
    ensure_daemon(true).await?;

    match open_socket().await? {
        Some((mut owned_read_half, mut owned_write_half)) => {
            // forward stdin to socket
            let mut stdin = io::stdin();
            let input_handle = rt.spawn(async move {
                loop {
                    match io::copy(&mut stdin, &mut owned_write_half).await {
                        Ok(b) => {
                            if b == 0 {
                                return Ok(());
                            }
                        }
                        Err(err) => return Err(err),
                    };
                }
            });

            // receive socket response to stdout
            let mut stdout = io::stdout();
            let out_put_handle = rt.spawn(async move {
                loop {
                    match io::copy(&mut owned_read_half, &mut stdout).await {
                        Ok(b) => {
                            if b == 0 {
                                return Ok(());
                            }
                        }
                        Err(err) => return Err(err),
                    };
                }
            });

            let _ = input_handle.await;
            let _ = out_put_handle.await;
            Ok(())
        }
        None => Ok(()),
    }
}

const fn log_file_name_prefix() -> &'static str {
    "server.log"
}

pub(crate) fn read_most_recent_log_file() -> io::Result<Option<String>> {
    let logs_dir = rome_log_dir();

    let most_recent = fs::read_dir(logs_dir)?
        .flatten()
        .filter(|file| file.file_type().map_or(false, |ty| ty.is_file()))
        .filter_map(|file| {
            match file
                .file_name()
                .to_str()?
                .split_once(log_file_name_prefix())
            {
                Some((_, date_part)) if date_part.split('-').count() == 4 => Some(file.path()),
                _ => None,
            }
        })
        .max();

    match most_recent {
        Some(file) => Ok(Some(fs::read_to_string(file)?)),
        None => Ok(None),
    }
}

/// Setup the [tracing]-based logging system for the server
/// The events received by the subscriber are filtered at the `info` level,
/// then printed using the [HierarchicalLayer] layer, and the resulting text
/// is written to log files rotated on a hourly basis (in
/// `rome-logs/server.log.yyyy-MM-dd-HH` files inside the system temporary
/// directory)
fn setup_tracing_subscriber() {
    let file_appender = tracing_appender::rolling::hourly(rome_log_dir(), log_file_name_prefix());

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

pub(super) fn rome_log_dir() -> PathBuf {
    match env::var_os("ROME_LOG_DIR") {
        Some(directory) => PathBuf::from(directory),
        None => env::temp_dir().join("rome-logs"),
    }
}

/// Tracing filter enabling:
/// - All spans and events at level info or higher
/// - All spans and events at level debug in crates whose name starts with `rome`
struct LoggingFilter;

/// Tracing filter used for spans emitted by `rome*` crates
const SELF_FILTER: LevelFilter = if cfg!(debug_assertions) {
    LevelFilter::TRACE
} else {
    LevelFilter::DEBUG
};

impl LoggingFilter {
    fn is_enabled(&self, meta: &Metadata<'_>) -> bool {
        let filter = if meta.target().starts_with("rome") {
            SELF_FILTER
        } else {
            LevelFilter::INFO
        };

        meta.level() <= &filter
    }
}

impl<S> Filter<S> for LoggingFilter {
    fn enabled(&self, meta: &Metadata<'_>, _cx: &Context<'_, S>) -> bool {
        self.is_enabled(meta)
    }

    fn callsite_enabled(&self, meta: &'static Metadata<'static>) -> Interest {
        if self.is_enabled(meta) {
            Interest::always()
        } else {
            Interest::never()
        }
    }

    fn max_level_hint(&self) -> Option<LevelFilter> {
        Some(SELF_FILTER)
    }
}
