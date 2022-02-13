use lspower::jsonrpc::Error as LspError;
use lspower::jsonrpc::Result as LspResult;
use lspower::lsp::*;
use lspower::{Client, LanguageServer, LspService, Server};
use rome_analyze::AnalysisServer;
use std::sync::Arc;
use tokio::io::{Stdin, Stdout};

use crate::capabilities::server_capabilities;
use crate::handlers;
use crate::handlers::formatting::{to_format_options, FormatOnTypeParams, FormatRangeParams};
use crate::line_index::LineIndex;
use crate::session::into_lsp_error;
use crate::session::Session;

struct LSPServer {
    client: Client,
    session: Arc<Session>,
}

impl LSPServer {
    fn new(client: Client) -> Self {
        let session = Arc::new(Session::new(client.clone()));
        Self { client, session }
    }

    pub async fn task<F, R>(&self, f: F) -> LspResult<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        tokio::task::spawn_blocking(f)
            .await
            .map_err(|_| LspError::internal_error())
    }
}

#[lspower::async_trait]
impl LanguageServer for LSPServer {
    async fn initialize(&self, _: InitializeParams) -> LspResult<InitializeResult> {
        let init = InitializeResult {
            capabilities: server_capabilities(),
            ..Default::default()
        };
        Ok(init)
    }

    async fn initialized(&self, _: InitializedParams) {
        let msg = format!("Server initialized with PID: {}", std::process::id());
        self.client.log_message(MessageType::INFO, msg).await;
    }

    async fn shutdown(&self) -> LspResult<()> {
        Ok(())
    }

    async fn code_action(&self, params: CodeActionParams) -> LspResult<Option<CodeActionResponse>> {
        let url = params.text_document.uri.clone();
        let doc = self.session.document(&url)?;

        let line_index = LineIndex::new(&doc.text);
        let cursor_range = crate::utils::text_range(&line_index, params.range);

        let mut analysis_server = AnalysisServer::default();
        analysis_server.set_file_text(doc.file_id, doc.text);

        let actions = self
            .task(move || {
                handlers::analysis::code_actions(analysis_server, doc.file_id, url, cursor_range)
            })
            .await??
            .into_iter()
            .map(CodeActionOrCommand::CodeAction)
            .collect();

        Ok(Some(actions))
    }

    async fn formatting(
        &self,
        params: DocumentFormattingParams,
    ) -> LspResult<Option<Vec<TextEdit>>> {
        let url = params.text_document.uri;
        let doc = self.session.document(&url)?;

        let edits = self
            .task(move || handlers::formatting::format(&doc.text, doc.file_id, &params.options))
            .await?
            .map_err(into_lsp_error)?;
        Ok(Some(edits))
    }

    async fn range_formatting(
        &self,
        params: DocumentRangeFormattingParams,
    ) -> LspResult<Option<Vec<TextEdit>>> {
        let url = params.text_document.uri;
        let doc = self.session.document(&url)?;

        let edits = self
            .task(move || {
                handlers::formatting::format_range(FormatRangeParams {
                    text: doc.text.as_ref(),
                    file_id: doc.file_id,
                    format_options: to_format_options(&params.options),
                    range: params.range,
                })
            })
            .await?
            .map_err(into_lsp_error)?;
        Ok(Some(edits))
    }

    async fn on_type_formatting(
        &self,
        params: DocumentOnTypeFormattingParams,
    ) -> LspResult<Option<Vec<TextEdit>>> {
        let url = params.text_document_position.text_document.uri;
        let doc = self.session.document(&url)?;

        let edits = self
            .task(move || {
                handlers::formatting::format_on_type(FormatOnTypeParams {
                    text: doc.text.as_ref(),
                    file_id: doc.file_id,
                    format_options: to_format_options(&params.options),
                    position: params.text_document_position.position,
                })
            })
            .await?
            .map_err(into_lsp_error)?;
        Ok(Some(edits))
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let session = self.session.clone();
        handlers::text_document::did_open(session, params).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let session = self.session.clone();
        handlers::text_document::did_change(session, params).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let session = self.session.clone();
        handlers::text_document::did_close(session, params).await;
    }
}

pub async fn run_server(stdin: Stdin, stdout: Stdout) {
    let (service, messages) = LspService::new(LSPServer::new);
    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}
