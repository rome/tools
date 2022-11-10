use std::{
    convert::Infallible,
    env,
    fs::read_dir,
    io::{self, ErrorKind},
    mem::swap,
    os::windows::process::CommandExt,
    pin::Pin,
    process::Command,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

use rome_lsp::{ServerConnection, ServerFactory};
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    net::windows::named_pipe::{ClientOptions, NamedPipeClient, NamedPipeServer, ServerOptions},
    time,
};
use tracing::Instrument;

/// Returns the name of the global named pipe used to communicate with the
/// server daemon
fn get_pipe_name() -> String {
    format!(r"\\.\pipe\rome-service-{}", rome_service::VERSION)
}

pub(crate) fn enumerate_pipes() -> io::Result<impl Iterator<Item = String>> {
    read_dir(r"\\.\pipe").map(|iter| {
        iter.filter_map(|entry| {
            let entry = entry.ok()?.path();
            let file_name = entry.file_name()?;
            let file_name = file_name.to_str()?;

            let rome_version = file_name.strip_prefix("rome-service")?;
            if rome_version.is_empty() {
                Some(String::new())
            } else {
                Some(rome_version.strip_prefix('-')?.to_string())
            }
        })
    })
}

/// Error code from the Win32 API
const ERROR_PIPE_BUSY: i32 = 231;

/// Try to connect to the global pipe and wait for the connection to become ready
async fn try_connect() -> io::Result<NamedPipeClient> {
    loop {
        match ClientOptions::new().open(get_pipe_name()) {
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
fn spawn_daemon(is_oneshot: bool) -> io::Result<()> {
    let binary = env::current_exe()?;

    let mut cmd = Command::new(binary);
    cmd.arg("__run_server");

    if is_oneshot {
        cmd.arg("--oneshot");
    }

    cmd.creation_flags(CREATE_NEW_PROCESS_GROUP);

    cmd.spawn()?;

    Ok(())
}

/// Open a connection to the daemon server process, returning [None] if the
/// server is not running
pub(crate) async fn open_socket() -> io::Result<Option<(ClientReadHalf, ClientWriteHalf)>> {
    match try_connect().await {
        Ok(socket) => {
            let inner = Arc::new(socket);
            Ok(Some((
                ClientReadHalf {
                    inner: inner.clone(),
                },
                ClientWriteHalf { inner },
            )))
        }
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}

pub(crate) struct ClientReadHalf {
    inner: Arc<NamedPipeClient>,
}

impl AsyncRead for ClientReadHalf {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        loop {
            match self.inner.poll_read_ready(cx) {
                Poll::Ready(Ok(())) => match self.inner.try_read(buf.initialize_unfilled()) {
                    Ok(count) => {
                        buf.advance(count);
                        return Poll::Ready(Ok(()));
                    }

                    Err(err) if err.kind() == io::ErrorKind::WouldBlock => continue,
                    Err(err) => return Poll::Ready(Err(err)),
                },

                Poll::Ready(Err(err)) => return Poll::Ready(Err(err)),
                Poll::Pending => return Poll::Pending,
            };
        }
    }
}

pub(crate) struct ClientWriteHalf {
    inner: Arc<NamedPipeClient>,
}

impl AsyncWrite for ClientWriteHalf {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        loop {
            match self.inner.poll_write_ready(cx) {
                Poll::Ready(Ok(())) => match self.inner.try_write(buf) {
                    Ok(count) => return Poll::Ready(Ok(count)),
                    Err(err) if err.kind() == io::ErrorKind::WouldBlock => continue,
                    Err(err) => return Poll::Ready(Err(err)),
                },

                Poll::Ready(Err(err)) => return Poll::Ready(Err(err)),
                Poll::Pending => return Poll::Pending,
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        self.poll_flush(cx)
    }
}

/// Ensure the server daemon is running and ready to receive connections
///
/// Returns false if the daemon process was already running or true if it had
/// to be started
pub(crate) async fn ensure_daemon(is_oneshot: bool) -> io::Result<bool> {
    let mut did_spawn = false;

    loop {
        match open_socket().await {
            Ok(Some(_)) => break,
            Ok(None) => {
                spawn_daemon(is_oneshot)?;
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
    ensure_daemon(true).await?;
    println!("{}", get_pipe_name());
    Ok(())
}

/// Start listening on the global pipe and accepting connections with the
/// provided [ServerFactory]
pub(crate) async fn run_daemon(factory: ServerFactory) -> io::Result<Infallible> {
    let mut prev_server = ServerOptions::new()
        .first_pipe_instance(true)
        .create(get_pipe_name())?;

    loop {
        prev_server.connect().await?;
        let mut next_server = ServerOptions::new().create(get_pipe_name())?;
        swap(&mut prev_server, &mut next_server);

        let connection = factory.create();
        let span = tracing::trace_span!("run_server");
        tokio::spawn(run_server(connection, next_server).instrument(span.or_current()));
    }
}

/// Async task driving a single client connection
async fn run_server(connection: ServerConnection, stream: NamedPipeServer) {
    let inner = Arc::new(stream);
    let read = ServerReadHalf {
        inner: inner.clone(),
    };
    let write = ServerWriteHalf { inner };
    connection.accept(read, write).await;
}

struct ServerReadHalf {
    inner: Arc<NamedPipeServer>,
}

impl AsyncRead for ServerReadHalf {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        loop {
            match self.inner.poll_read_ready(cx) {
                Poll::Ready(Ok(())) => match self.inner.try_read(buf.initialize_unfilled()) {
                    Ok(count) => {
                        buf.advance(count);
                        return Poll::Ready(Ok(()));
                    }

                    Err(err) if err.kind() == io::ErrorKind::WouldBlock => continue,
                    Err(err) => return Poll::Ready(Err(err)),
                },

                Poll::Ready(Err(err)) => return Poll::Ready(Err(err)),
                Poll::Pending => return Poll::Pending,
            };
        }
    }
}

struct ServerWriteHalf {
    inner: Arc<NamedPipeServer>,
}

impl AsyncWrite for ServerWriteHalf {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        loop {
            match self.inner.poll_write_ready(cx) {
                Poll::Ready(Ok(())) => match self.inner.try_write(buf) {
                    Ok(count) => return Poll::Ready(Ok(count)),
                    Err(err) if err.kind() == io::ErrorKind::WouldBlock => continue,
                    Err(err) => return Poll::Ready(Err(err)),
                },

                Poll::Ready(Err(err)) => return Poll::Ready(Err(err)),
                Poll::Pending => return Poll::Pending,
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        self.poll_flush(cx)
    }
}
