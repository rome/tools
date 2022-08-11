//! Implements the OS dependent transport layer for the server protocol. This
//! uses a domain socket created in the global temporary directory on Unix
//! systems, and a named pipe on Windows. The protocol used for message frames
//! is based on the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#baseProtocol),
//! a simplified derivative of the HTTP protocol

use std::{io, panic::RefUnwindSafe, str::FromStr};

use anyhow::{bail, ensure, Context, Error};
use rome_service::{workspace::WorkspaceTransport, TransportError};
use tokio::{
    io::{split, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader},
    runtime::Runtime,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use self::windows::open_socket;
#[cfg(windows)]
pub(crate) use self::windows::{print_socket, run_daemon};

#[cfg(unix)]
mod unix;
#[cfg(unix)]
use self::unix::open_socket;
#[cfg(unix)]
pub(crate) use self::unix::{print_socket, run_daemon};

/// Tries to open a connection to a running daemon instance, returning a
/// [WorkspaceTransport] instance if the socket is currently active
pub(crate) fn open_transport(runtime: Runtime) -> io::Result<Option<impl WorkspaceTransport>> {
    match runtime.block_on(open_socket()) {
        Ok(Some(socket)) => Ok(Some(SocketTransport::open(runtime, socket))),
        Ok(None) => Ok(None),
        Err(err) => Err(err),
    }
}

/// Implementation of [WorkspaceTransport] for types implementing [AsyncRead]
/// and [AsyncWrite]
pub struct SocketTransport {
    runtime: Runtime,
    read_recv: UnboundedReceiver<Vec<u8>>,
    write_send: UnboundedSender<Vec<u8>>,
}

impl SocketTransport {
    pub fn open<T>(runtime: Runtime, socket: T) -> Self
    where
        T: AsyncRead + AsyncWrite + Send + 'static,
    {
        let (socket_read, mut socket_write) = split(socket);
        let mut socket_read = BufReader::new(socket_read);

        let (read_send, read_recv) = mpsc::unbounded_channel();
        let (write_send, mut write_recv) = mpsc::unbounded_channel::<Vec<_>>();

        let read_task = async move {
            loop {
                let mut length = None;
                let mut line = String::new();

                loop {
                    match socket_read
                        .read_line(&mut line)
                        .await
                        .context("failed to read header line from the socket")?
                    {
                        // A read of 0 bytes means the connection was closed
                        0 => {
                            bail!("the connection to the remote workspace was unexpectedly closed");
                        }
                        // A read of two bytes corresponds to the "\r\n" sequence
                        // that indicates the end of the header section
                        2 => {
                            if line != "\r\n" {
                                bail!("unexpected byte sequence received from the remote workspace, got {line:?} expected \"\\r\\n\"");
                            }

                            break;
                        }
                        _ => {
                            let header: TransportHeader = line
                                .parse()
                                .context("failed to parse header from the remote workspace")?;

                            match header {
                                TransportHeader::ContentLength(value) => {
                                    length = Some(value);
                                }
                                TransportHeader::ContentType => {}
                                TransportHeader::Unknown(name) => {
                                    eprintln!("ignoring unknown header {name:?}");
                                }
                            }

                            line.clear();
                        }
                    }
                }

                let length = length.context("incoming response from the remote workspace is missing the Content-Length header")?;

                let mut result = vec![0u8; length];
                socket_read.read_exact(&mut result).await.with_context(|| {
                    format!("failed to read message of {length} bytes from the socket")
                })?;

                // Send the received message over the transport channel, or
                // exit the task if the channel was closed
                if read_send.send(result).is_err() {
                    break;
                }
            }

            Ok(())
        };

        let write_task = async move {
            while let Some(message) = write_recv.recv().await {
                socket_write.write_all(b"Content-Length: ").await?;

                let length = message.len().to_string();
                socket_write.write_all(length.as_bytes()).await?;
                socket_write.write_all(b"\r\n").await?;

                socket_write
                    .write_all(b"Content-Type: application/vscode-jsonrpc; charset=utf-8\r\n")
                    .await?;

                socket_write.write_all(b"\r\n").await?;

                socket_write.write_all(&message).await?;
            }

            Ok::<(), Error>(())
        };

        runtime.spawn(async move {
            if let Err(err) = read_task.await {
                eprintln!(
                    "{:?}",
                    err.context("remote connection read task exited with an error")
                );
            }
        });

        runtime.spawn(async move {
            if let Err(err) = write_task.await {
                eprintln!(
                    "{:?}",
                    err.context("remote connection write task exited with an error")
                );
            }
        });

        Self {
            runtime,
            read_recv,
            write_send,
        }
    }
}

// Allow the socket to be recovered across panic boundaries
impl RefUnwindSafe for SocketTransport {}

impl WorkspaceTransport for SocketTransport {
    fn send(&mut self, request: Vec<u8>) -> Result<(), TransportError> {
        self.write_send
            .send(request)
            .map_err(|_| TransportError::ChannelClosed)
    }

    fn receive(&mut self) -> Result<Vec<u8>, TransportError> {
        let read_recv = &mut self.read_recv;
        self.runtime
            .block_on(async move { read_recv.recv().await.ok_or(TransportError::ChannelClosed) })
    }
}

enum TransportHeader {
    ContentLength(usize),
    ContentType,
    Unknown(String),
}

impl FromStr for TransportHeader {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let colon = line
            .find(':')
            .with_context(|| format!("could not find colon token in {line:?}"))?;

        let (name, value) = line.split_at(colon);
        let value = value[1..].trim();

        match name {
            "Content-Length" => {
                let value = value.parse().with_context(|| {
                    format!("could not parse Content-Length header value {value:?}")
                })?;

                Ok(TransportHeader::ContentLength(value))
            }
            "Content-Type" => {
                ensure!(
                    value.starts_with( "application/vscode-jsonrpc"),
                    "invalid value for Content-Type expected \"application/vscode-jsonrpc\", got {value:?}"
                );

                Ok(TransportHeader::ContentType)
            }
            _ => Ok(TransportHeader::Unknown(name.into())),
        }
    }
}
