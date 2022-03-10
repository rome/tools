use lspower::jsonrpc::Error as LspError;
use lspower::jsonrpc::Result as LspResult;
use lspower::lsp::*;
use lspower::{Client, LanguageServer, LspService, Server};
use rome_analyze::AnalysisServer;
use std::sync::Arc;
use tokio::io::{Stdin, Stdout};
use tracing::{error, info, trace};

use crate::capabilities::server_capabilities;
use crate::config::CONFIGURATION_SECTION;
use crate::handlers;
use crate::handlers::formatting::{
    to_format_options, FormatOnTypeParams, FormatParams, FormatRangeParams,
};
use crate::line_index::LineIndex;
use crate::session::Session;
use crate::utils;

struct LSPServer {
    client: Client,
    session: Arc<Session>,
}

impl LSPServer {
    fn new(client: Client) -> Self {
        let session = Arc::new(Session::new(client.clone()));
        Self { client, session }
    }
}

#[lspower::async_trait]
impl LanguageServer for LSPServer {
    async fn initialize(&self, params: InitializeParams) -> LspResult<InitializeResult> {
        info!("Starting Rome Language Server...");

        if let Some(value) = params.initialization_options {
            self.session
                .config
                .write()
                .set_workspace_settings(value)
                .map_err(|err| {
                    error!("Cannot set workspace settings: {}", err);
                    LspError::internal_error()
                })?;
        }

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
        let item = ConfigurationItem {
            scope_uri: None,
            section: Some(String::from(CONFIGURATION_SECTION)),
        };
        let items = vec![item];
        let configurations = self.client.configuration(items).await;

        if let Ok(configurations) = configurations {
            configurations.into_iter().next().and_then(|configuration| {
                self.session
                    .config
                    .write()
                    .set_workspace_settings(configuration)
                    .map_err(|err| {
                        error!("Cannot set workspace settings: {}", err);
                    })
                    .ok()
            });
        } else {
            trace!("Cannot read configuration from the client");
        }

        let msg = format!("Server initialized with PID: {}", std::process::id());
        self.client.log_message(MessageType::INFO, msg).await;
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

        let line_index = LineIndex::new(&doc.text);
        let cursor_range = crate::utils::text_range(&line_index, params.range);

        let file_id = doc.file_id();
        let mut analysis_server = AnalysisServer::default();
        analysis_server.set_file_text(file_id, doc.text);

        let task = utils::spawn_blocking_task(move || {
            handlers::analysis::code_actions(analysis_server, file_id, url, cursor_range)
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

        let task = utils::spawn_blocking_task(move || {
            handlers::formatting::format(FormatParams {
                text: &doc.text,
                source_type: doc.get_source_type(),
                format_options: to_format_options(&params.options),
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
                format_options: to_format_options(&params.options),
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
                format_options: to_format_options(&params.options),
                position: params.text_document_position.position,
                workspace_settings,
                source_type: doc.get_source_type(),
            })
        });
        let edits = task.await?;
        Ok(edits)
    }

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        let result = self
            .session
            .config
            .write()
            .set_workspace_settings(params.settings);

        if let Err(err) = result {
            error!("Cannot set workspace settings: {}", err);
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
    let (service, messages) = LspService::new(LSPServer::new);
    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}
