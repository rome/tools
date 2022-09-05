use std::sync::Arc;

use crate::capabilities::server_capabilities;
use crate::requests::syntax_tree::{SyntaxTreePayload, SYNTAX_TREE_REQUEST};
use crate::session::Session;
use crate::utils::{into_lsp_error, panic_to_lsp_error};
use crate::{handlers, requests};
use futures::future::ready;
use futures::FutureExt;
use rome_service::{workspace, Workspace};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::Notify;
use tokio::task::spawn_blocking;
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::{lsp_types::*, ClientSocket};
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tracing::{error, info, trace};

pub struct LSPServer {
    session: Session,
}

impl LSPServer {
    fn new(client: Client, workspace: Arc<dyn Workspace>, cancellation: Arc<Notify>) -> Self {
        Self {
            session: Session::new(client, workspace, cancellation),
        }
    }

    async fn syntax_tree_request(&self, params: SyntaxTreePayload) -> LspResult<String> {
        trace!(
            "Calling method: {}\n with params: {:?}",
            SYNTAX_TREE_REQUEST,
            &params
        );

        let url = params.text_document.uri;
        requests::syntax_tree::syntax_tree(&self.session, &url).map_err(into_lsp_error)
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for LSPServer {
    #[tracing::instrument(level = "trace", skip(self))]
    async fn initialize(&self, params: InitializeParams) -> LspResult<InitializeResult> {
        info!("Starting Rome Language Server...");

        self.session
            .client_capabilities
            .write()
            .unwrap()
            .replace(params.capabilities);

        if let Some(uri) = params.root_uri {
            self.session.root_uri.write().unwrap().replace(uri);
        }

        let init = InitializeResult {
            capabilities: server_capabilities(),
            server_info: Some(ServerInfo {
                name: String::from(env!("CARGO_PKG_NAME")),
                version: Some(String::from(env!("CARGO_PKG_VERSION"))),
            }),
        };

        Ok(init)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn initialized(&self, params: InitializedParams) {
        let _ = params;

        info!("Attempting to load the configuration from 'rome.json' file");

        self.session.update_configuration().await;
        self.session.fetch_client_configuration().await;

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

    #[tracing::instrument(level = "trace", skip(self))]
    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        let _ = params;
        self.session.fetch_client_configuration().await;
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

/// Factory data structure responsible for creating [ServerConnection] handles
/// for each incoming connection accepted by the server
#[derive(Default)]
pub struct ServerFactory {
    /// Synchronisation primitve used to broadcast a shutdown signal to all
    /// active connections
    cancellation: Arc<Notify>,
    /// Optional [Workspace] instance shared between all clients. Currently
    /// this field is always [None] (meaning each connection will get its own
    /// workspace) until we figure out how to handle concurrent access to the
    /// same workspace from multiple client
    workspace: Option<Arc<dyn Workspace>>,
}

/// Helper method for wrapping a [Workspace] method in a `custom_method` for
/// the [LSPServer]
macro_rules! workspace_method {
    ( $builder:ident, $method:ident ) => {
        $builder = $builder.custom_method(
            concat!("rome/", stringify!($method)),
            |server: &LSPServer, params| {
                let span = tracing::trace_span!(concat!("rome/", stringify!($method)), params = ?params).or_current();

                let workspace = server.session.workspace.clone();
                let result = spawn_blocking(move || {
                    let _guard = span.entered();
                    workspace.$method(params)
                });

                result.map(move |result| {
                    match result {
                        Ok(Ok(result)) => Ok(result),
                        Ok(Err(err)) => Err(into_lsp_error(err)),
                        Err(err) => match err.try_into_panic() {
                            Ok(err) => Err(panic_to_lsp_error(err)),
                            Err(err) => Err(into_lsp_error(err)),
                        },
                    }
                })
            },
        );
    };
}

impl ServerFactory {
    /// Create a new [ServerConnection] from this factory
    pub fn create(&self) -> ServerConnection {
        let workspace = self
            .workspace
            .clone()
            .unwrap_or_else(workspace::server_sync);

        let mut builder = LspService::build(move |client| {
            LSPServer::new(client, workspace, self.cancellation.clone())
        });

        builder = builder.custom_method(SYNTAX_TREE_REQUEST, LSPServer::syntax_tree_request);

        // "shutdown" is not part of the Workspace API
        builder = builder.custom_method("rome/shutdown", |server: &LSPServer, (): ()| {
            tracing::info!("Sending shutdown signal");
            server.session.broadcast_shutdown();
            ready(Ok(Some(())))
        });

        workspace_method!(builder, supports_feature);
        workspace_method!(builder, update_settings);
        workspace_method!(builder, open_file);
        workspace_method!(builder, get_syntax_tree);
        workspace_method!(builder, get_control_flow_graph);
        workspace_method!(builder, get_formatter_ir);
        workspace_method!(builder, change_file);
        workspace_method!(builder, close_file);
        workspace_method!(builder, pull_diagnostics);
        workspace_method!(builder, pull_actions);
        workspace_method!(builder, format_file);
        workspace_method!(builder, format_range);
        workspace_method!(builder, format_on_type);
        workspace_method!(builder, fix_file);
        workspace_method!(builder, rename);

        let (service, socket) = builder.finish();
        ServerConnection { socket, service }
    }

    /// Return a handle to the cancellation token for this server process
    pub fn cancellation(&self) -> Arc<Notify> {
        self.cancellation.clone()
    }
}

/// Handle type created by the server for each incoming connection
pub struct ServerConnection {
    socket: ClientSocket,
    service: LspService<LSPServer>,
}

impl ServerConnection {
    /// Destructure a connection into its inner service instance and socket
    pub fn into_inner(self) -> (LspService<LSPServer>, ClientSocket) {
        (self.service, self.socket)
    }

    /// Accept an incoming connection and run the server async I/O loop to
    /// completion
    pub async fn accept<I, O>(self, stdin: I, stdout: O)
    where
        I: AsyncRead + Unpin,
        O: AsyncWrite,
    {
        Server::new(stdin, stdout, self.socket)
            .serve(self.service)
            .await;
    }
}
