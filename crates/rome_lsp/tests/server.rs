use anyhow::bail;
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use futures::channel::mpsc::{channel, Sender};
use futures::Sink;
use futures::SinkExt;
use futures::Stream;
use futures::StreamExt;
use rome_fs::RomePath;
use rome_lsp::LSPServer;
use rome_lsp::ServerFactory;
use rome_lsp::WorkspaceSettings;
use rome_service::workspace::GetSyntaxTreeParams;
use rome_service::workspace::GetSyntaxTreeResult;
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
use tower_lsp::lsp_types as lsp;
use tower_lsp::lsp_types::DidChangeTextDocumentParams;
use tower_lsp::lsp_types::DidCloseTextDocumentParams;
use tower_lsp::lsp_types::DidOpenTextDocumentParams;
use tower_lsp::lsp_types::DocumentFormattingParams;
use tower_lsp::lsp_types::FormattingOptions;
use tower_lsp::lsp_types::InitializeResult;
use tower_lsp::lsp_types::InitializedParams;
use tower_lsp::lsp_types::Position;
use tower_lsp::lsp_types::PublishDiagnosticsParams;
use tower_lsp::lsp_types::Range;
use tower_lsp::lsp_types::TextDocumentContentChangeEvent;
use tower_lsp::lsp_types::TextDocumentIdentifier;
use tower_lsp::lsp_types::TextDocumentItem;
use tower_lsp::lsp_types::TextEdit;
use tower_lsp::lsp_types::VersionedTextDocumentIdentifier;
use tower_lsp::lsp_types::WorkDoneProgressParams;
use tower_lsp::lsp_types::{ClientCapabilities, CodeDescription, Url};
use tower_lsp::LspService;
use tower_lsp::{jsonrpc::Request, lsp_types::InitializeParams};

/// Statically build an [lsp::Url] instance that points to the file at `$path`
/// within the workspace. The filesystem path contained in the return URI is
/// guaranteed to be a valid path for the underlying operating system, but
/// doesn't have to refer to an existing file on the host machine.
macro_rules! url {
    ($path:literal) => {
        if cfg!(windows) {
            lsp::Url::parse(concat!("file:///z%3A/workspace/", $path)).unwrap()
        } else {
            lsp::Url::parse(concat!("file:///workspace/", $path)).unwrap()
        }
    };
}

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
                    root_uri: Some(url!("")),
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
                    uri: url!("document.js"),
                    language_id: String::from("javascript"),
                    version: 0,
                    text: text.to_string(),
                },
            },
        )
        .await
    }

    /// Opens a document with given contents and given name. The name must contain the extension too
    async fn open_named_document(
        &mut self,
        text: impl Display,
        document_name: Url,
        language: impl Display,
    ) -> Result<()> {
        self.notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: document_name,
                    language_id: language.to_string(),
                    version: 0,
                    text: text.to_string(),
                },
            },
        )
        .await
    }

    async fn change_document(
        &mut self,
        version: i32,
        content_changes: Vec<TextDocumentContentChangeEvent>,
    ) -> Result<()> {
        self.notify(
            "textDocument/didChange",
            DidChangeTextDocumentParams {
                text_document: VersionedTextDocumentIdentifier {
                    uri: url!("document.js"),
                    version,
                },
                content_changes,
            },
        )
        .await
    }

    async fn close_document(&mut self) -> Result<()> {
        self.notify(
            "textDocument/didClose",
            DidCloseTextDocumentParams {
                text_document: TextDocumentIdentifier {
                    uri: url!("document.js"),
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

/// Number of notifications buffered by the server-to-client channel before it starts blocking the current task
const CHANNEL_BUFFER_SIZE: usize = 8;

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

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
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

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .open_document("first_line();\nsecond_line();\nthird_line();")
        .await?;

    server
        .change_document(
            1,
            vec![
                TextDocumentContentChangeEvent {
                    range: Some(Range {
                        start: Position {
                            line: 2,
                            character: 6,
                        },
                        end: Position {
                            line: 2,
                            character: 10,
                        },
                    }),
                    range_length: None,
                    text: String::from("statement"),
                },
                TextDocumentContentChangeEvent {
                    range: Some(Range {
                        start: Position {
                            line: 1,
                            character: 7,
                        },
                        end: Position {
                            line: 1,
                            character: 11,
                        },
                    }),
                    range_length: None,
                    text: String::from("statement"),
                },
                TextDocumentContentChangeEvent {
                    range: Some(Range {
                        start: Position {
                            line: 0,
                            character: 6,
                        },
                        end: Position {
                            line: 0,
                            character: 10,
                        },
                    }),
                    range_length: None,
                    text: String::from("statement"),
                },
            ],
        )
        .await?;

    let res: GetSyntaxTreeResult = server
        .request(
            "rome/get_syntax_tree",
            "get_syntax_tree",
            GetSyntaxTreeParams {
                path: RomePath::new("document.js"),
            },
        )
        .await?
        .expect("get_syntax_tree returned None");

    const EXPECTED: &str = "0: JS_MODULE@0..57
  0: (empty)
  1: JS_DIRECTIVE_LIST@0..0
  2: JS_MODULE_ITEM_LIST@0..57
    0: JS_EXPRESSION_STATEMENT@0..18
      0: JS_CALL_EXPRESSION@0..17
        0: JS_IDENTIFIER_EXPRESSION@0..15
          0: JS_REFERENCE_IDENTIFIER@0..15
            0: IDENT@0..15 \"first_statement\" [] []
        1: (empty)
        2: (empty)
        3: JS_CALL_ARGUMENTS@15..17
          0: L_PAREN@15..16 \"(\" [] []
          1: JS_CALL_ARGUMENT_LIST@16..16
          2: R_PAREN@16..17 \")\" [] []
      1: SEMICOLON@17..18 \";\" [] []
    1: JS_EXPRESSION_STATEMENT@18..38
      0: JS_CALL_EXPRESSION@18..37
        0: JS_IDENTIFIER_EXPRESSION@18..35
          0: JS_REFERENCE_IDENTIFIER@18..35
            0: IDENT@18..35 \"second_statement\" [Newline(\"\\n\")] []
        1: (empty)
        2: (empty)
        3: JS_CALL_ARGUMENTS@35..37
          0: L_PAREN@35..36 \"(\" [] []
          1: JS_CALL_ARGUMENT_LIST@36..36
          2: R_PAREN@36..37 \")\" [] []
      1: SEMICOLON@37..38 \";\" [] []
    2: JS_EXPRESSION_STATEMENT@38..57
      0: JS_CALL_EXPRESSION@38..56
        0: JS_IDENTIFIER_EXPRESSION@38..54
          0: JS_REFERENCE_IDENTIFIER@38..54
            0: IDENT@38..54 \"third_statement\" [Newline(\"\\n\")] []
        1: (empty)
        2: (empty)
        3: JS_CALL_ARGUMENTS@54..56
          0: L_PAREN@54..55 \"(\" [] []
          1: JS_CALL_ARGUMENT_LIST@55..55
          2: R_PAREN@55..56 \")\" [] []
      1: SEMICOLON@56..57 \";\" [] []
  3: EOF@57..57 \"\" [] []
";

    assert_eq!(res.cst, EXPECTED);

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

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: url!("document"),
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
                    uri: url!("document"),
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
                    uri: url!("document"),
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

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
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
                uri: url!("document.js"),
                version: Some(0),
                diagnostics: vec![lsp::Diagnostic {
                    range: lsp::Range {
                        start: lsp::Position {
                            line: 0,
                            character: 5,
                        },
                        end: lsp::Position {
                            line: 0,
                            character: 7,
                        },
                    },
                    severity: Some(lsp::DiagnosticSeverity::ERROR),
                    code: Some(lsp::NumberOrString::String(String::from(
                        "lint/suspicious/noDoubleEquals",
                    ))),
                    code_description: Some(CodeDescription {
                        href: Url::parse("https://docs.rome.tools/lint/rules/noDoubleEquals")
                            .unwrap()
                    }),
                    source: Some(String::from("rome")),
                    message: String::from(
                        "Use === instead of ==.\n== is only allowed when comparing against `null`",
                    ),
                    related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                        location: lsp::Location {
                            uri: url!("document.js"),
                            range: lsp::Range {
                                start: lsp::Position {
                                    line: 0,
                                    character: 5,
                                },
                                end: lsp::Position {
                                    line: 0,
                                    character: 7,
                                },
                            },
                        },
                        message: String::new(),
                    }]),
                    tags: None,
                    data: None,
                }],
            }
        ))
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

fn fixable_diagnostic(line: u32) -> Result<lsp::Diagnostic> {
    Ok(lsp::Diagnostic {
        range: lsp::Range {
            start: lsp::Position { line, character: 3 },
            end: lsp::Position {
                line,
                character: 11,
            },
        },
        severity: Some(lsp::DiagnosticSeverity::ERROR),
        code: Some(lsp::NumberOrString::String(String::from(
            "lint/suspicious/noCompareNegZero",
        ))),
        code_description: None,
        source: Some(String::from("rome")),
        message: String::from("Do not use the === operator to compare against -0."),
        related_information: None,
        tags: None,
        data: None,
    })
}

#[tokio::test]
async fn pull_quick_fixes() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document("if(a === -0) {}").await?;

    let res: lsp::CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            lsp::CodeActionParams {
                text_document: lsp::TextDocumentIdentifier {
                    uri: url!("document.js"),
                },
                range: lsp::Range {
                    start: lsp::Position {
                        line: 0,
                        character: 6,
                    },
                    end: lsp::Position {
                        line: 0,
                        character: 6,
                    },
                },
                context: lsp::CodeActionContext {
                    diagnostics: vec![fixable_diagnostic(0)?],
                    only: Some(vec![lsp::CodeActionKind::QUICKFIX]),
                },
                work_done_progress_params: lsp::WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: lsp::PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    let mut changes = HashMap::default();
    changes.insert(
        url!("document.js"),
        vec![lsp::TextEdit {
            range: lsp::Range {
                start: lsp::Position {
                    line: 0,
                    character: 9,
                },
                end: lsp::Position {
                    line: 0,
                    character: 10,
                },
            },
            new_text: String::new(),
        }],
    );

    let expected_code_action = lsp::CodeActionOrCommand::CodeAction(lsp::CodeAction {
        title: String::from("Replace -0 with 0"),
        kind: Some(lsp::CodeActionKind::new(
            "quickfix.rome.suspicious.noCompareNegZero",
        )),
        diagnostics: Some(vec![fixable_diagnostic(0)?]),
        edit: Some(lsp::WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    });

    let mut suppression_changes = HashMap::default();
    suppression_changes.insert(
        url!("document.js"),
        vec![lsp::TextEdit {
            range: lsp::Range {
                start: lsp::Position {
                    line: 0,
                    character: 0,
                },
                end: lsp::Position {
                    line: 0,
                    character: 0,
                },
            },
            new_text: String::from(
                "// rome-ignore lint/suspicious/noCompareNegZero: <explanation>\n",
            ),
        }],
    );

    let expected_suppression_action = lsp::CodeActionOrCommand::CodeAction(lsp::CodeAction {
        title: String::from("Suppress rule lint/suspicious/noCompareNegZero"),
        kind: Some(lsp::CodeActionKind::new(
            "quickfix.suppressRule.rome.suspicious.noCompareNegZero",
        )),
        diagnostics: Some(vec![fixable_diagnostic(0)?]),
        edit: Some(lsp::WorkspaceEdit {
            changes: Some(suppression_changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: None,
        disabled: None,
        data: None,
    });

    assert_eq!(res, vec![expected_code_action, expected_suppression_action]);

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_diagnostics_for_rome_json() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let incorrect_config = r#"{
        "formatter": {
            "indentStyle": "magic"
        }
    }"#;
    server
        .open_named_document(incorrect_config, url!("rome.json"), "json")
        .await?;

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
                uri: url!("rome.json"),
                version: Some(0),
                diagnostics: vec![lsp::Diagnostic {
                    range: lsp::Range {
                        start: lsp::Position {
                            line: 2,
                            character: 27,
                        },
                        end: lsp::Position {
                            line: 2,
                            character: 34,
                        },
                    },
                    severity: Some(lsp::DiagnosticSeverity::ERROR),
                    code: Some(lsp::NumberOrString::String(String::from("configuration",))),
                    code_description: None,
                    source: Some(String::from("rome")),
                    message: String::from("Found an unknown value `magic`",),
                    related_information: None,
                    tags: None,
                    data: None,
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
async fn pull_refactors() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .open_document("let variable = \"value\"; func(variable);")
        .await?;

    let res: lsp::CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            lsp::CodeActionParams {
                text_document: lsp::TextDocumentIdentifier {
                    uri: url!("document.js"),
                },
                range: lsp::Range {
                    start: lsp::Position {
                        line: 0,
                        character: 7,
                    },
                    end: lsp::Position {
                        line: 0,
                        character: 7,
                    },
                },
                context: lsp::CodeActionContext {
                    diagnostics: vec![],
                    only: Some(vec![lsp::CodeActionKind::REFACTOR]),
                },
                work_done_progress_params: lsp::WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: lsp::PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    let mut changes = HashMap::default();

    changes.insert(
        url!("document.js"),
        vec![
            lsp::TextEdit {
                range: lsp::Range {
                    start: lsp::Position {
                        line: 0,
                        character: 0,
                    },
                    end: lsp::Position {
                        line: 0,
                        character: 15,
                    },
                },
                new_text: String::from("func("),
            },
            lsp::TextEdit {
                range: lsp::Range {
                    start: lsp::Position {
                        line: 0,
                        character: 22,
                    },
                    end: lsp::Position {
                        line: 0,
                        character: 37,
                    },
                },
                new_text: String::new(),
            },
        ],
    );

    let expected_action = lsp::CodeActionOrCommand::CodeAction(lsp::CodeAction {
        title: String::from("Inline variable"),
        kind: Some(lsp::CodeActionKind::new(
            "refactor.inline.rome.correctness.inlineVariable",
        )),
        diagnostics: None,
        edit: Some(lsp::WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    });

    assert_eq!(res, vec![expected_action]);

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_fix_all() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .open_document("if(a === -0) {}\nif(a === -0) {}\nif(a === -0) {}")
        .await?;

    let res: lsp::CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            lsp::CodeActionParams {
                text_document: lsp::TextDocumentIdentifier {
                    uri: url!("document.js"),
                },
                range: lsp::Range {
                    start: lsp::Position {
                        line: 0,
                        character: 7,
                    },
                    end: lsp::Position {
                        line: 0,
                        character: 7,
                    },
                },
                context: lsp::CodeActionContext {
                    diagnostics: vec![
                        fixable_diagnostic(0)?,
                        fixable_diagnostic(1)?,
                        fixable_diagnostic(2)?,
                    ],
                    only: Some(vec![lsp::CodeActionKind::new("source.fixAll")]),
                },
                work_done_progress_params: lsp::WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: lsp::PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    let mut changes = HashMap::default();

    changes.insert(
        url!("document.js"),
        vec![lsp::TextEdit {
            range: lsp::Range {
                start: lsp::Position {
                    line: 0,
                    character: 0,
                },
                end: lsp::Position {
                    line: 3,
                    character: 0,
                },
            },
            new_text: String::from("if(a === 0) {}\nif(a === 0) {}\nif(a === 0) {}"),
        }],
    );

    let expected_action = lsp::CodeActionOrCommand::CodeAction(lsp::CodeAction {
        title: String::from("Fix all auto-fixable issues"),
        kind: Some(lsp::CodeActionKind::new("source.fixAll.rome")),
        diagnostics: Some(vec![
            fixable_diagnostic(0)?,
            fixable_diagnostic(1)?,
            fixable_diagnostic(2)?,
        ]),
        edit: Some(lsp::WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    });

    assert_eq!(res, vec![expected_action]);

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

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
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
                    uri: url!("document.js"),
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

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
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
