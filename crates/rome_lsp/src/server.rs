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
use rome_analyze::AnalysisServer;
use rome_js_parser::parse;
use rome_js_parser::symbols::Symbol;
use rome_js_syntax::{TextRange, TextSize};
use serde_json::Value;
use std::sync::Arc;
use tokio::io::{Stdin, Stdout};
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tracing::{debug, error, info, trace};

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

pub fn line_starts(source: &'_ str) -> impl '_ + Iterator<Item = TextSize> {
    std::iter::once(0)
        .chain(source.match_indices(&['\n', '\r']).filter_map(|(i, _)| {
            let bytes = source.as_bytes();

            match bytes[i] {
                // Filter out the `\r` in `\r\n` to avoid counting the line break twice
                b'\r' if i + 1 < bytes.len() && bytes[i + 1] == b'\n' => None,
                _ => Some(i + 1),
            }
        }))
        .map(|i| TextSize::try_from(i).expect("integer overflow"))
}

fn position_to_offset(line_starts: &Vec<TextSize>, position: &Position) -> Option<TextSize> {
    line_starts[position.line as usize].checked_add(position.character.into())
}

fn offset_to_position(line_starts: &Vec<TextSize>, offset: &TextSize) -> Option<Position> {
    let next_line_idx = line_starts
        .iter()
        .position(|x| x > offset)
        .unwrap_or(line_starts.len());

    line_starts
        .get(next_line_idx - 1)
        .map(|line_start_offset| Position {
            line: (next_line_idx - 1) as u32,
            character: (offset - line_start_offset).into(),
        })
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

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> tower_lsp::jsonrpc::Result<Option<GotoDefinitionResponse>> {
        //GotoDefinitionParams { text_document_position_params: TextDocumentPositionParams { text_document: TextDocumentIdentifier { uri: Url { scheme: "file", cannot_be_a_base: false, username: "", password: None, host: None, port: None, path: "/c%3A/myprojects/chatserver/front/chat/main.js", query: None, fragment: None } }, position: Position { line: 3, character: 8 } }, work_done_progress_params: WorkDoneProgressParams { work_done_token: None }, partial_result_params: PartialResultParams { partial_result_token: None } }
        trace!("Got a textDocument/definition request: {:?}", params);

        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .clone();
        let document = self.session.document(&uri)?;

        let text = &document.text;
        let line_starts = line_starts(text).collect::<Vec<_>>();

        let symbols = tokio::task::spawn_blocking(move || {
            info!("Goto definition");
            trace!("Goto definition for: {:?}", document);
            let text = &document.text;
            let file_id = document.file_id();
            let source_type = document.get_source_type();
            let parse_result = parse(text, file_id, source_type);
            let symbols =
                rome_js_parser::symbols::symbols(parse_result.syntax()).collect::<Vec<_>>();
            symbols
        })
        .await
        .unwrap();
        debug!("Symbols: {:?}", symbols);

        let offset =
            position_to_offset(&line_starts, &params.text_document_position_params.position)
                .unwrap();
        let symbol = symbols.iter().find(|s| s.range().contains(offset));
        match symbol {
            Some(Symbol::Reference { declared_at, .. }) if declared_at.is_some() => {
                let declared_at = declared_at.unwrap();
                let start = offset_to_position(&line_starts, &declared_at.start()).unwrap();
                let end = offset_to_position(&line_starts, &declared_at.end()).unwrap();
                Ok(Some(GotoDefinitionResponse::Scalar(Location {
                    uri: uri.clone(),
                    range: Range { start, end },
                })))
            }
            _ => Ok(None),
        }
    }
}

pub async fn run_server(stdin: Stdin, stdout: Stdout) {
    let (service, messages) = LspService::build(LSPServer::new)
        .custom_method(SYNTAX_TREE_REQUEST, LSPServer::syntax_tree_request)
        .finish();
    Server::new(stdin, stdout, messages).serve(service).await;
}
