use std::sync::Arc;

use lspower::jsonrpc::Result;
use lspower::lsp::*;
use lspower::{Client, LanguageServer, LspService, Server};
use rome_analyze::{AnalysisServer, FileId, Signal, TextAction};
use rome_formatter::IndentStyle;
use rslint_parser::TextRange;
use tokio::io::{Stdin, Stdout};
use tokio::sync::Mutex;
use tracing::{debug, error, trace, trace_span, Instrument};

use crate::capabilities::server_capabilities;
use crate::documents::{Document, DocumentStore};
use crate::handlers::{self, FormatOnTypeParams, FormatRangeParams};
use crate::line_index::LineIndex;
use crate::url_interner::UrlInterner;
use crate::utils::{self, text_action_to_lsp};

struct LSPServer {
    client: Client,
    state: Arc<Mutex<State>>,
}

#[derive(Default)]
struct State {
    text_documents: DocumentStore,
    url_interner: UrlInterner,
}

impl LSPServer {
    fn new(client: Client) -> Self {
        Self {
            client,
            state: Default::default(),
        }
    }

    async fn file_id(&self, url: Url) -> FileId {
        self.state.lock().await.url_interner.intern(url)
    }

    async fn url(&self, file_id: FileId) -> Url {
        self.state
            .lock()
            .await
            .url_interner
            .lookup(file_id)
            .to_owned()
    }

    async fn text(&self, file_id: FileId) -> Option<Arc<String>> {
        self.state
            .lock()
            .await
            .text_documents
            .get(&file_id)
            .map(|d| d.text.clone())
    }

    async fn document(&self, file_id: FileId) -> Option<Arc<Document>> {
        self.state.lock().await.text_documents.get(&file_id)
    }

    async fn update_diagnostics(&self, file_id: FileId) {
        let url = self.url(file_id).await;
        let doc = match self.document(file_id).await {
            Some(doc) => doc,
            None => {
                error!("Missing document while computing diagnostics for: {}", url);
                return;
            }
        };
        let text = doc.text.clone();
        let client = self.client.clone();

        let span = trace_span!("Analyzing file", ?file_id);
        let handle = tokio::spawn(
            async move {
                analyze_file(file_id, text).await.unwrap_or_else(|e| {
                    error!("Error while computing diagnostics: {}", e);
                    (vec![], vec![])
                })
            }
            .instrument(span),
        );

        match handle.await {
            Ok((diagnostics, actions)) => {
                client.publish_diagnostics(url, diagnostics, None).await;
                self.set_actions(file_id, actions, doc.version).await;
            }
            Err(e) => error!("Error while joining analysis thread: {}", e),
        }
    }

    async fn set_actions(
        &self,
        file_id: FileId,
        actions: impl Into<Arc<Vec<TextAction>>>,
        version: i32,
    ) {
        self.state
            .lock()
            .await
            .text_documents
            .update_actions(file_id, actions, version)
    }
}

async fn analyze_file(
    file_id: FileId,
    text: impl Into<Arc<String>>,
) -> anyhow::Result<(Vec<Diagnostic>, Vec<TextAction>)> {
    trace!("Computing");

    let text = text.into();

    let line_index = LineIndex::new(&text);
    let mut analysis_server = AnalysisServer::default();
    analysis_server.set_file_text(file_id, text);

    let mut actions: Vec<TextAction> = vec![];
    let mut diagnostics: Vec<Diagnostic> = vec![];

    let analysis = analysis_server.analyze(file_id);
    for signal in analysis.signals {
        match signal {
            Signal::Diagnostic(d) => {
                if let Some(diagnostic) = utils::diagnostic_to_lsp(d.diagnostic, &line_index) {
                    diagnostics.push(diagnostic);
                    actions.extend(d.actions.into_iter().map(TextAction::from));
                }
            }
            Signal::Action(a) => actions.push(a.into()),
        }
    }

    Ok((diagnostics, actions))
}

fn compute_actions(
    uri: Url,
    file_id: FileId,
    text: impl Into<Arc<String>>,
    cursor_range: TextRange,
) -> Result<Vec<CodeAction>> {
    let text = text.into();
    let line_index = LineIndex::new(&text);
    let mut analysis_api = AnalysisServer::default();
    analysis_api.set_file_text(file_id, text);

    let code_actions: Vec<CodeAction> = analysis_api
        .assists(file_id, cursor_range)
        .into_actions()
        .map(|a| utils::text_action_to_lsp(&a.into(), &line_index, uri.to_owned(), None))
        .collect();

    Ok(code_actions)
}

#[lspower::async_trait]
impl LanguageServer for LSPServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        let init = InitializeResult {
            capabilities: server_capabilities(),
            ..Default::default()
        };
        Ok(init)
    }

    async fn initialized(&self, _: InitializedParams) {
        let msg = format!("Server initialized with PID: {}", std::process::id());
        debug!("Server started.");
        self.client.log_message(MessageType::INFO, msg).await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        trace!("Code action request");

        let uri = params.text_document.uri.clone();
        let file_id = self.file_id(uri.clone()).await;
        let text = self.text(file_id).await.unwrap();

        let line_index = LineIndex::new(&text);
        let cursor_range = crate::utils::text_range(&line_index, params.range);

        trace!("Code action cursor range: {:?}", cursor_range);

        let span = trace_span!("Computing Actions");
        let handle = tokio::spawn(
            async move { compute_actions(uri, file_id, text, cursor_range) }.instrument(span),
        );

        let doc = self
            .document(file_id)
            .await
            .expect("File not in document store");

        let mut actions: Vec<_> = doc
            .code_actions
            .iter()
            .filter(|a| a.target.contains_range(cursor_range))
            .map(|a| text_action_to_lsp(a, &line_index, params.text_document.uri.clone(), None))
            .map(CodeActionOrCommand::CodeAction)
            .collect();

        // TODO: Clean up this error handling
        match handle.await {
            Ok(res) => match res {
                Ok(a) => actions.extend(a.into_iter().map(CodeActionOrCommand::CodeAction)),

                Err(e) => error!("Error while computing code actions: {}", e),
            },
            Err(e) => error!("Error while joining thread: {}", e),
        };

        Ok(Some(actions))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let url = params.text_document.uri.clone();
        trace!("Formatting: {:?}", url);

        let file_id;
        let text;
        {
            let mut state = self.state.lock().await;
            file_id = state.url_interner.intern(url);
            text = state.text_documents.get(&file_id).unwrap().text.clone();
        }
        let handle = tokio::spawn(async move { handlers::format(&text, file_id) });

        // TODO: Clean up this error handling
        let opt = match handle.await {
            Ok(res) => match res {
                Ok(edits) => Some(edits),
                Err(e) => {
                    error!("Error while formatting: {}", e);
                    None
                }
            },
            Err(e) => {
                error!("Error while joining thread: {}", e);
                None
            }
        };
        Ok(opt)
    }

    async fn range_formatting(
        &self,
        params: DocumentRangeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let url = params.text_document.uri.clone();
        trace!("Formatting: {:?}", url);

        let file_id;
        let text;
        {
            let mut state = self.state.lock().await;
            file_id = state.url_interner.intern(url);
            text = state.text_documents.get(&file_id).unwrap().text.clone();
        }

        let handle = tokio::spawn(async move {
            handlers::format_range(FormatRangeParams {
                text: text.as_ref(),
                file_id,
                indent_style: if params.options.insert_spaces {
                    IndentStyle::Space(params.options.tab_size as u8)
                } else {
                    IndentStyle::Tab
                },
                range: params.range,
            })
        });

        // TODO: Clean up this error handling
        let opt = match handle.await {
            Ok(res) => match res {
                Ok(edits) => Some(edits),
                Err(e) => {
                    error!("Error while formatting: {}", e);
                    None
                }
            },
            Err(e) => {
                error!("Error while joining thread: {}", e);
                None
            }
        };

        Ok(opt)
    }

    async fn on_type_formatting(
        &self,
        params: DocumentOnTypeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let url = params.text_document_position.text_document.uri.clone();
        trace!("Formatting: {:?}", url);

        let file_id;
        let text;
        {
            let mut state = self.state.lock().await;
            file_id = state.url_interner.intern(url);
            text = state.text_documents.get(&file_id).unwrap().text.clone();
        }

        let handle = tokio::spawn(async move {
            handlers::format_on_type(FormatOnTypeParams {
                text: text.as_ref(),
                file_id,
                indent_style: if params.options.insert_spaces {
                    IndentStyle::Space(params.options.tab_size as u8)
                } else {
                    IndentStyle::Tab
                },
                position: params.text_document_position.position,
            })
        });

        // TODO: Clean up this error handling
        let opt = match handle.await {
            Ok(res) => match res {
                Ok(edits) => Some(edits),
                Err(e) => {
                    error!("Error while formatting: {}", e);
                    None
                }
            },
            Err(e) => {
                error!("Error while joining thread: {}", e);
                None
            }
        };

        Ok(opt)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let url = params.text_document.uri.clone();
        trace!("Did Open: {}", url);

        let file_id;
        {
            let mut state = self.state.lock().await;
            file_id = state.url_interner.intern(url.clone());
            let text = Arc::new(params.text_document.text);
            state
                .text_documents
                .set(file_id, text, params.text_document.version);
        }
        let span = trace_span!("Initial Diagnostics");
        self.update_diagnostics(file_id).instrument(span).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let url = params.text_document.uri.clone();
        trace!("Did Change: {}", url);

        let mut content_changes = params.content_changes;

        // Because of TextDocumentSyncKind::Full, there should only be one change.
        let text = match content_changes.pop() {
            Some(change) => change.text,
            None => {
                error!(
                    "Content change missing in textDocument/didChange for {:?}",
                    url
                );
                return;
            }
        };

        let file_id;
        {
            let mut state = self.state.lock().await;
            file_id = state.url_interner.intern(url.clone());

            state
                .text_documents
                .update_text(file_id, Arc::new(text), params.text_document.version);
        }
        let span = trace_span!("Updating Diagnostics");
        self.update_diagnostics(file_id).instrument(span).await;
    }
}

pub async fn run_server(stdin: Stdin, stdout: Stdout) {
    let (service, messages) = LspService::new(LSPServer::new);
    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}
