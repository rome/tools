use std::{
    convert::Infallible,
    env,
    io::{self, ErrorKind},
    mem::swap,
    os::windows::process::CommandExt,
    process::Command,
    time::Duration,
};

use rome_lsp::{ServerConnection, ServerFactory};
use tokio::{
    io::split,
    net::windows::named_pipe::{ClientOptions, NamedPipeClient, NamedPipeServer, ServerOptions},
    time,
};
use tracing::Instrument;

/// Name of the global named pipe used to communicate with the server daemon
const PIPE_NAME: &str = r"\\.\pipe\rome-service";

/// Error code from the Win32 API
const ERROR_PIPE_BUSY: i32 = 231;

/// Try to connect to the global pipe and wait for the connection to become ready
async fn try_connect() -> io::Result<NamedPipeClient> {
    loop {
        match ClientOptions::new().open(PIPE_NAME) {
            Ok(client) => return Ok(client),
            // If the connection failed with ERROR_PIPE_BUSY, wait a few
            // milliseconds then retry the connection (we should be using
            // WaitNamedPipe here but that's not exposed by tokio / mio)
            Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY) => {}
            Err(e) => return Err(e),
        }

        time::sleep(Duration::from_millis(50)).await;
    }
}

/// Process creationg flag from the Win32 API, ensures the process is created
/// in its own group and will not be killed when the parent process exits
const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;

/// Spawn the daemon server process in the background
fn spawn_daemon() -> io::Result<()> {
    let binary = env::current_exe()?;

    let mut cmd = Command::new(binary);
    cmd.arg("__run_server");

    cmd.creation_flags(CREATE_NEW_PROCESS_GROUP);

    cmd.spawn()?;

    Ok(())
}

/// Open a connection to the daemon server process, returning [None] if the
/// server is not running
pub(crate) async fn open_socket() -> io::Result<Option<NamedPipeClient>> {
    match try_connect().await {
        Ok(socket) => Ok(Some(socket)),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}

/// Ensure the server daemon is running and ready to receive connections
///
/// Returns false if the daemon process was already running or true if it had
/// to be started
pub(crate) async fn ensure_daemon() -> io::Result<bool> {
    let mut did_spawn = false;

    loop {
        match open_socket().await {
            Ok(Some(_)) => break,
            Ok(None) => {
                spawn_daemon()?;
                did_spawn = true;
                time::sleep(Duration::from_millis(50)).await;
            }
            Err(err) => return Err(err),
        }
    }

    Ok(did_spawn)
}

/// Ensure the server daemon is running and ready to receive connections and
/// print the global pipe name in the standard output
pub(crate) async fn print_socket() -> io::Result<()> {
    ensure_daemon().await?;
    println!("{PIPE_NAME}");
    Ok(())
}

/// Start listening on the global pipe and accepting connections with the
/// provided [ServerFactory]
pub(crate) async fn run_daemon(factory: ServerFactory) -> io::Result<Infallible> {
    let mut prev_server = ServerOptions::new()
        .first_pipe_instance(true)
        .create(PIPE_NAME)?;

    loop {
        prev_server.connect().await?;
        let mut next_server = ServerOptions::new().create(PIPE_NAME)?;
        swap(&mut prev_server, &mut next_server);

        let connection = factory.create();
        let span = tracing::trace_span!("run_server");
        tokio::spawn(run_server(connection, next_server).instrument(span.or_current()));
    }
}

/// Async task driving a single client connection
async fn run_server(connection: ServerConnection, stream: NamedPipeServer) {
    let (read, write) = split(stream);
    connection.accept(read, write).await;
}
