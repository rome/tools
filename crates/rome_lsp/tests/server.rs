use anyhow::bail;
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use futures::channel::mpsc::{channel, Sender};
use futures::Sink;
use futures::SinkExt;
use futures::Stream;
use futures::StreamExt;
use rome_lsp::LSPServer;
use rome_lsp::ServerFactory;
use rome_lsp::WorkspaceSettings;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_value, to_value};
use std::any::type_name;
use std::collections::HashMap;
use std::fmt::Display;
use std::slice;
use std::time::Duration;
use tokio::time::sleep;
use tower::timeout::Timeout;
use tower::{Service, ServiceExt};
use tower_lsp::jsonrpc;
use tower_lsp::jsonrpc::Response;
use tower_lsp::lsp_types::ClientCapabilities;
use tower_lsp::lsp_types::CodeActionContext;
use tower_lsp::lsp_types::CodeActionKind;
use tower_lsp::lsp_types::CodeActionOrCommand;
use tower_lsp::lsp_types::CodeActionParams;
use tower_lsp::lsp_types::CodeActionResponse;
use tower_lsp::lsp_types::Diagnostic;
use tower_lsp::lsp_types::DiagnosticRelatedInformation;
use tower_lsp::lsp_types::DiagnosticSeverity;
use tower_lsp::lsp_types::DidCloseTextDocumentParams;
use tower_lsp::lsp_types::DidOpenTextDocumentParams;
use tower_lsp::lsp_types::DocumentFormattingParams;
use tower_lsp::lsp_types::FormattingOptions;
use tower_lsp::lsp_types::InitializeResult;
use tower_lsp::lsp_types::InitializedParams;
use tower_lsp::lsp_types::Location;
use tower_lsp::lsp_types::NumberOrString;
use tower_lsp::lsp_types::PartialResultParams;
use tower_lsp::lsp_types::Position;
use tower_lsp::lsp_types::PublishDiagnosticsParams;
use tower_lsp::lsp_types::Range;
use tower_lsp::lsp_types::TextDocumentIdentifier;
use tower_lsp::lsp_types::TextDocumentItem;
use tower_lsp::lsp_types::TextEdit;
use tower_lsp::lsp_types::Url;
use tower_lsp::lsp_types::WorkDoneProgressParams;
use tower_lsp::LspService;
use tower_lsp::{jsonrpc::Request, lsp_types::InitializeParams};

struct Server {
    service: Timeout<LspService<LSPServer>>,
}

impl Server {
    fn new(service: LspService<LSPServer>) -> Self {
        Self {
            service: Timeout::new(service, Duration::from_secs(1)),
        }
    }

    async fn notify<P>(&mut self, method: &'static str, params: P) -> Result<()>
    where
        P: Serialize,
    {
        self.service
            .ready()
            .await
            .map_err(Error::msg)
            .context("ready() returned an error")?
            .call(
                Request::build(method)
                    .params(to_value(&params).context("failed to serialize params")?)
                    .finish(),
            )
            .await
            .map_err(Error::msg)
            .context("call() returned an error")
            .and_then(|res| {
                if let Some(res) = res {
                    bail!("shutdown returned {:?}", res)
                } else {
                    Ok(())
                }
            })
    }

    async fn request<P, R>(
        &mut self,
        method: &'static str,
        id: &'static str,
        params: P,
    ) -> Result<Option<R>>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        self.service
            .ready()
            .await
            .map_err(Error::msg)
            .context("ready() returned an error")?
            .call(
                Request::build(method)
                    .id(id)
                    .params(to_value(&params).context("failed to serialize params")?)
                    .finish(),
            )
            .await
            .map_err(Error::msg)
            .context("call() returned an error")?
            .map(|res| {
                let (_, body) = res.into_parts();

                let body =
                    body.with_context(|| format!("response to {method:?} contained an error"))?;

                from_value(body.clone()).with_context(|| {
                    format!(
                        "failed to deserialize type {} from response {body:?}",
                        type_name::<R>()
                    )
                })
            })
            .transpose()
    }

    /// Basic implementation of the `initialize` request for tests
    // The `root_path` field is deprecated, but we still need to specify it
    #[allow(deprecated)]
    async fn initialize(&mut self) -> Result<()> {
        let _res: InitializeResult = self
            .request(
                "initialize",
                "_init",
                InitializeParams {
                    process_id: None,
                    root_path: None,
                    root_uri: None,
                    initialization_options: None,
                    capabilities: ClientCapabilities::default(),
                    trace: None,
                    workspace_folders: None,
                    client_info: None,
                    locale: None,
                },
            )
            .await?
            .context("initialize returned None")?;

        Ok(())
    }

    /// Basic implementation of the `initialized` notification for tests
    async fn initialized(&mut self) -> Result<()> {
        self.notify("initialized", InitializedParams {}).await
    }

    /// Basic implementation of the `shutdown` notification for tests
    async fn shutdown(&mut self) -> Result<()> {
        self.service
            .ready()
            .await
            .map_err(Error::msg)
            .context("ready() returned an error")?
            .call(Request::build("shutdown").finish())
            .await
            .map_err(Error::msg)
            .context("call() returned an error")
            .and_then(|res| {
                if let Some(res) = res {
                    bail!("shutdown returned {:?}", res)
                } else {
                    Ok(())
                }
            })
    }

    async fn open_document(&mut self, text: impl Display) -> Result<()> {
        self.notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: Url::parse("test://workspace/document.js")?,
                    language_id: String::from("javascript"),
                    version: 0,
                    text: text.to_string(),
                },
            },
        )
        .await
    }

    async fn close_document(&mut self) -> Result<()> {
        self.notify(
            "textDocument/didClose",
            DidCloseTextDocumentParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::parse("test://workspace/document.js")?,
                },
            },
        )
        .await
    }

    /// Basic implementation of the `rome/shutdown` request for tests
    async fn rome_shutdown(&mut self) -> Result<()> {
        self.request::<_, ()>("rome/shutdown", "_rome_shutdown", ())
            .await?
            .context("rome/shutdown returned None")?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ServerNotification {
    PublishDiagnostics(PublishDiagnosticsParams),
}

/// Basic handler for requests and notifications coming from the server for tests
async fn client_handler<I, O>(
    mut stream: I,
    mut sink: O,
    mut notify: Sender<ServerNotification>,
) -> Result<()>
where
    // This function has to be generic as `RequestStream` and `ResponseSink`
    // are not exported from `tower_lsp` and cannot be named in the signature
    I: Stream<Item = Request> + Unpin,
    O: Sink<Response> + Unpin,
{
    while let Some(req) = stream.next().await {
        if req.method() == "textDocument/publishDiagnostics" {
            let params = req.params().expect("invalid request");
            let diagnostics = from_value(params.clone()).expect("invalid params");
            let notification = ServerNotification::PublishDiagnostics(diagnostics);
            match notify.send(notification).await {
                Ok(_) => continue,
                Err(_) => break,
            }
        }

        let id = match req.id() {
            Some(id) => id,
            None => continue,
        };

        let res = match req.method() {
            "workspace/configuration" => {
                let settings = WorkspaceSettings {
                    ..WorkspaceSettings::default()
                };

                let result =
                    to_value(slice::from_ref(&settings)).context("failed to serialize settings")?;

                Response::from_ok(id.clone(), result)
            }
            _ => Response::from_error(id.clone(), jsonrpc::Error::method_not_found()),
        };

        sink.send(res).await.ok();
    }

    Ok(())
}

#[tokio::test]
async fn basic_lifecycle() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(8);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn document_lifecycle() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(8);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document("").await?;
    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn document_no_extension() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(8);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: Url::parse("test://workspace/document")?,
                    language_id: String::from("javascript"),
                    version: 0,
                    text: String::from("statement()"),
                },
            },
        )
        .await?;

    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::parse("test://workspace/document")?,
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    let edits = res.context("formatting did not return an edit list")?;
    assert!(!edits.is_empty(), "formatting returned an empty edit list");

    server
        .notify(
            "textDocument/didClose",
            DidCloseTextDocumentParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::parse("test://workspace/document")?,
                },
            },
        )
        .await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_diagnostics() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(8);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document("if(a == b) {}").await?;

    let notification = tokio::select! {
        msg = receiver.next() => msg,
        _ = sleep(Duration::from_secs(1)) => {
            panic!("timed out waiting for the server to send diagnostics")
        }
    };

    assert_eq!(
        notification,
        Some(ServerNotification::PublishDiagnostics(
            PublishDiagnosticsParams {
                uri: Url::parse("test://workspace/document.js")?,
                version: Some(0),
                diagnostics: vec![Diagnostic {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 5
                        },
                        end: Position {
                            line: 0,
                            character: 7
                        }
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String(String::from(
                        "lint/correctness/noDoubleEquals"
                    ))),
                    code_description: None,
                    source: Some(String::from("rome")),
                    message: String::from(
                        "Use === instead of ==.\n== is only allowed when comparing against `null`"
                    ),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            uri: Url::parse("test://workspace/document.js")?,
                            range: Range {
                                start: Position {
                                    line: 0,
                                    character: 5
                                },
                                end: Position {
                                    line: 0,
                                    character: 7
                                }
                            }
                        },
                        message: String::new()
                    }]),
                    tags: None,
                    data: None
                }],
            }
        ))
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_quick_fixes() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(8);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document("if(a == b) {}").await?;

    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::parse("test://workspace/document.js")?,
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 6,
                    },
                    end: Position {
                        line: 0,
                        character: 6,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![],
                    only: Some(vec![CodeActionKind::QUICKFIX]),
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    let code_actions: Vec<_> = res
        .iter()
        .map(|action| match action {
            CodeActionOrCommand::Command(_) => panic!("unexpected command"),
            CodeActionOrCommand::CodeAction(action) => &action.title,
        })
        .collect();

    assert_eq!(code_actions.as_slice(), &["Use ==="]);

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_refactors() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(8);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .open_document("let variable = \"value\"; func(variable);")
        .await?;

    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::parse("test://workspace/document.js")?,
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 7,
                    },
                    end: Position {
                        line: 0,
                        character: 7,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![],
                    only: Some(vec![CodeActionKind::REFACTOR]),
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    let code_actions: Vec<_> = res
        .iter()
        .map(|action| match action {
            CodeActionOrCommand::Command(_) => panic!("unexpected command"),
            CodeActionOrCommand::CodeAction(action) => &action.title,
        })
        .collect();

    assert_eq!(code_actions.as_slice(), &["Inline variable"]);

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn format_with_syntax_errors() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(8);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document("expression(").await?;

    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::parse("test://workspace/document.js")?,
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    assert!(res.is_none());

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn server_shutdown() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(8);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let cancellation = factory.cancellation();
    let cancellation = cancellation.notified();

    server.rome_shutdown().await?;

    cancellation.await;

    reader.abort();

    Ok(())
}
