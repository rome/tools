use crate::capabilities::server_capabilities;
use crate::requests::syntax_tree::{SyntaxTreePayload, SYNTAX_TREE_REQUEST};
use crate::session::Session;
use crate::utils::into_lsp_error;
use crate::{handlers, requests};
use rome_service::{load_config, ConfigurationType};
use serde_json::Value;
use tokio::io::{Stdin, Stdout};
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::{lsp_types::*, ClientSocket};
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tracing::{error, info, trace};

pub struct LSPServer {
    session: Session,
}

impl LSPServer {
    fn new(client: Client) -> Self {
        let session = Session::new(client);
        Self { session }
    }

    async fn syntax_tree_request(&self, params: SyntaxTreePayload) -> LspResult<Option<Value>> {
        trace!(
            "Calling method: {}\n with params: {:?}",
            SYNTAX_TREE_REQUEST,
            &params
        );

        let url = params.text_document.uri;
        let result =
            requests::syntax_tree::syntax_tree(&self.session, &url).map_err(into_lsp_error)?;

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
            server_info: Some(ServerInfo {
                name: String::from(env!("CARGO_PKG_NAME")),
                version: Some(String::from(env!("CARGO_PKG_VERSION"))),
            }),
        };

        Ok(init)
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("Attempting to load the configuration from 'rome.json' file");

        match load_config(&self.session.fs, ConfigurationType::Root) {
            Ok(Some(configuration)) => {
                info!("Configuration found, and it is valid!");
                self.session.configuration.write().replace(configuration);
            }
            Err(err) => {
                error!("Couldn't load the configuration file, reason:\n {}", err);
            }
            _ => {}
        };

        self.session.fetch_client_configuration().await;

        if self.session.config.read().get_workspace_settings().unstable {
            rome_flags::set_unstable_flags(rome_flags::FeatureFlags::ALL);
        }

        let msg = format!("Server initialized with PID: {}", std::process::id());
        self.session
            .client
            .log_message(MessageType::INFO, msg)
            .await;

        let mut registrations = Vec::new();

        if self.session.can_register_did_change_configuration() {
            registrations.push(Registration {
                id: "workspace/didChangeConfiguration".to_string(),
                method: "workspace/didChangeConfiguration".to_string(),
                register_options: None,
            });
        }

        if !registrations.is_empty() {
            if let Err(e) = self.session.client.register_capability(registrations).await {
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
        handlers::analysis::code_actions(&self.session, params).map_err(into_lsp_error)
    }

    async fn formatting(
        &self,
        params: DocumentFormattingParams,
    ) -> LspResult<Option<Vec<TextEdit>>> {
        handlers::formatting::format(&self.session, params).map_err(into_lsp_error)
    }

    async fn range_formatting(
        &self,
        params: DocumentRangeFormattingParams,
    ) -> LspResult<Option<Vec<TextEdit>>> {
        handlers::formatting::format_range(&self.session, params).map_err(into_lsp_error)
    }

    async fn on_type_formatting(
        &self,
        params: DocumentOnTypeFormattingParams,
    ) -> LspResult<Option<Vec<TextEdit>>> {
        handlers::formatting::format_on_type(&self.session, params).map_err(into_lsp_error)
    }

    async fn did_change_configuration(&self, _params: DidChangeConfigurationParams) {
        let diags_enabled_prev = self.session.diagnostics_enabled();

        self.session.fetch_client_configuration().await;

        if diags_enabled_prev != self.session.diagnostics_enabled() {
            self.session.update_all_diagnostics().await;
        }
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        handlers::text_document::did_open(&self.session, params)
            .await
            .ok();
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        handlers::text_document::did_change(&self.session, params)
            .await
            .ok();
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        handlers::text_document::did_close(&self.session, params)
            .await
            .ok();
    }

    async fn rename(&self, params: RenameParams) -> LspResult<Option<WorkspaceEdit>> {
        handlers::rename::rename(&self.session, params).map_err(into_lsp_error)
    }
}

pub fn build_server() -> (LspService<LSPServer>, ClientSocket) {
    LspService::build(LSPServer::new)
        .custom_method(SYNTAX_TREE_REQUEST, LSPServer::syntax_tree_request)
        .finish()
}

pub async fn run_server(stdin: Stdin, stdout: Stdout) {
    let (service, messages) = build_server();
    Server::new(stdin, stdout, messages).serve(service).await;
}
