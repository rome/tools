use crate::capabilities::server_capabilities;
use crate::handlers;
use crate::handlers::formatting::{
    to_format_options, FormatOnTypeParams, FormatParams, FormatRangeParams,
};
use crate::line_index::LineIndex;
use crate::requests::syntax_tree::{syntax_tree, SyntaxTreePayload, SYNTAX_TREE_REQUEST};
use crate::session::Session;
use crate::utils;
use crate::utils::into_lsp_error;
use serde_json::Value;
use std::sync::Arc;
use tokio::io::{Stdin, Stdout};
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tracing::{error, info, trace};

struct LSPServer {
    client: Client,
    session: Arc<Session>,
}

impl LSPServer {
    fn new(client: Client) -> Self {
        let session = Arc::new(Session::new(client.clone()));
        Self { client, session }
    }

    async fn syntax_tree_request(&self, params: SyntaxTreePayload) -> LspResult<Option<Value>> {
        trace!(
            "Calling method: {}\n with params: {:?}",
            SYNTAX_TREE_REQUEST,
            &params
        );

        let params: SyntaxTreePayload = params;
        let url = params.text_document.uri.clone();
        let document = self.session.document(&url)?;
        let task = utils::spawn_blocking_task(move || syntax_tree(document));
        let result = task.await?;
        let result = serde_json::to_value(result).map_err(into_lsp_error)?;
        Ok(Some(result))
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for LSPServer {
    async fn initialize(&self, params: InitializeParams) -> LspResult<InitializeResult> {
        info!("Starting Rome Language Server...");

        self.session
            .client_capabilities
            .write()
            .replace(params.capabilities);

        let init = InitializeResult {
            capabilities: server_capabilities(),
            ..Default::default()
        };

        Ok(init)
    }

    async fn initialized(&self, _: InitializedParams) {
        self.session.fetch_client_configuration().await;

        if self.session.config.read().get_workspace_settings().unstable {
            rome_flags::set_unstable_flags(rome_flags::FeatureFlags::ALL);
        }

        let msg = format!("Server initialized with PID: {}", std::process::id());
        self.client.log_message(MessageType::INFO, msg).await;

        if self.session.can_register_did_change_configuration() {
            let registration = Registration {
                id: "workspace/didChangeConfiguration".to_string(),
                method: "workspace/didChangeConfiguration".to_string(),
                register_options: None,
            };

            if let Err(e) = self.client.register_capability(vec![registration]).await {
                error!("Error registering didChangeConfiguration capability: {}", e);
            }
        }

        // Diagnostics are disabled by default, so update them after fetching workspace config
        self.session.update_all_diagnostics().await;
    }

    async fn shutdown(&self) -> LspResult<()> {
        Ok(())
    }

    async fn code_action(&self, params: CodeActionParams) -> LspResult<Option<CodeActionResponse>> {
        let workspace_settings = self.session.config.read().get_workspace_settings();
        if !workspace_settings.analysis.enable_code_actions {
            return Ok(None);
        }

        let url = params.text_document.uri.clone();
        let doc = self.session.document(&url)?;
        let diagnostics = params.context.diagnostics;

        let line_index = LineIndex::new(&doc.text);
        let cursor_range = crate::utils::text_range(&line_index, params.range);

        let file_id = doc.file_id();

        let task = utils::spawn_blocking_task(move || {
            handlers::analysis::code_actions(file_id, &doc.text, url, &diagnostics, cursor_range)
        });
        let actions = task.await?;
        Ok(Some(actions))
    }

    async fn formatting(
        &self,
        params: DocumentFormattingParams,
    ) -> LspResult<Option<Vec<TextEdit>>> {
        let url = params.text_document.uri;
        let doc = self.session.document(&url)?;
        let workspace_settings = self.session.config.read().get_workspace_settings();

        trace!("Formatting...");
        let task = utils::spawn_blocking_task(move || {
            handlers::formatting::format(FormatParams {
                text: &doc.text,
                source_type: doc.get_source_type(),
                format_options: to_format_options(&params.options, &workspace_settings.formatter),
                workspace_settings,
                file_id: doc.file_id(),
            })
        });
        let edits = task.await?;
        Ok(edits)
    }

    async fn range_formatting(
        &self,
        params: DocumentRangeFormattingParams,
    ) -> LspResult<Option<Vec<TextEdit>>> {
        let url = params.text_document.uri;
        let doc = self.session.document(&url)?;
        let workspace_settings = self.session.config.read().get_workspace_settings();

        let task = utils::spawn_blocking_task(move || {
            handlers::formatting::format_range(FormatRangeParams {
                text: doc.text.as_ref(),
                file_id: doc.file_id(),
                format_options: to_format_options(&params.options, &workspace_settings.formatter),
                range: params.range,
                workspace_settings,
                source_type: doc.get_source_type(),
            })
        });
        let edits = task.await?;
        Ok(edits)
    }

    async fn on_type_formatting(
        &self,
        params: DocumentOnTypeFormattingParams,
    ) -> LspResult<Option<Vec<TextEdit>>> {
        let url = params.text_document_position.text_document.uri;
        let doc = self.session.document(&url)?;
        let workspace_settings = self.session.config.read().get_workspace_settings();

        let task = utils::spawn_blocking_task(move || {
            handlers::formatting::format_on_type(FormatOnTypeParams {
                text: doc.text.as_ref(),
                file_id: doc.file_id(),
                format_options: to_format_options(&params.options, &workspace_settings.formatter),
                position: params.text_document_position.position,
                workspace_settings,
                source_type: doc.get_source_type(),
            })
        });
        let edits = task.await?;
        Ok(edits)
    }

    async fn did_change_configuration(&self, _params: DidChangeConfigurationParams) {
        let diags_enabled_prev = self.session.diagnostics_enabled();

        self.session.fetch_client_configuration().await;

        if diags_enabled_prev != self.session.diagnostics_enabled() {
            self.session.update_all_diagnostics().await;
        }
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
    let (service, messages) = LspService::build(LSPServer::new)
        .custom_method(SYNTAX_TREE_REQUEST, LSPServer::syntax_tree_request)
        .finish();
    Server::new(stdin, stdout, messages).serve(service).await;
}
