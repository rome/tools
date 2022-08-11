use std::{io, panic::RefUnwindSafe};

use rome_service::{workspace::WorkspaceTransport, RomeError};
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
                    match socket_read.read_line(&mut line).await? {
                        // A read of 0 bytes means the connection was closed
                        0 => {
                            return Err(RomeError::IoError(io::Error::new(
                                io::ErrorKind::UnexpectedEof,
                                "unexpected EOF",
                            )));
                        }
                        // A read of two bytes corresponds to the "\r\n" sequence
                        // that indicates the end of the header section
                        2 => {
                            if line != "\r\n" {
                                return Err(RomeError::IoError(io::Error::new(
                                    io::ErrorKind::Other,
                                    "unexpected byte sequence",
                                )));
                            }

                            break;
                        }
                        _ => {
                            let colon = line.find(':').ok_or_else(|| {
                                RomeError::IoError(io::Error::new(
                                    io::ErrorKind::Other,
                                    "invalid header line",
                                ))
                            })?;

                            let (name, value) = line.split_at(colon);
                            let value = value[1..].trim();

                            match name {
                                "Content-Length" => {
                                    length = Some(value.parse().map_err(|err| {
                                        RomeError::IoError(io::Error::new(
                                            io::ErrorKind::Other,
                                            err,
                                        ))
                                    })?);
                                }
                                "Content-Type" => {
                                    if !value.starts_with("application/json") {
                                        return Err(RomeError::IoError(io::Error::new(
                                            io::ErrorKind::Other,
                                            "unsupported Content-Type encoding",
                                        )));
                                    }
                                }
                                _ => {
                                    eprintln!("ignoring unknown header {name:?}");
                                }
                            }

                            line.clear();
                        }
                    }
                }

                let length = length.ok_or_else(|| {
                    RomeError::IoError(io::Error::new(
                        io::ErrorKind::Other,
                        "incoming response is missing the Content-Length header",
                    ))
                })?;

                let mut result = vec![0u8; length];
                socket_read.read_exact(&mut result).await?;

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

            Ok::<(), io::Error>(())
        };

        runtime.spawn(async move {
            match read_task.await {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("read_task exited with error: {err}");
                }
            }
        });

        runtime.spawn(async move {
            match write_task.await {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("write_task exited with error: {err}");
                }
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
    fn send(&mut self, request: Vec<u8>) -> Result<(), RomeError> {
        self.write_send.send(request).map_err(|_| {
            RomeError::IoError(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "send channel was closed",
            ))
        })
    }

    fn receive(&mut self) -> Result<Vec<u8>, RomeError> {
        let read_recv = &mut self.read_recv;
        self.runtime.block_on(async move {
            read_recv.recv().await.ok_or_else(|| {
                RomeError::IoError(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "receive channel was closed",
                ))
            })
        })
    }
}
