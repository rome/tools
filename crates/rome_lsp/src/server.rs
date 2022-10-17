use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use crate::capabilities::server_capabilities;
use crate::requests::syntax_tree::{SyntaxTreePayload, SYNTAX_TREE_REQUEST};
use crate::session::{ClientInformation, Session, SessionHandle, SessionKey};
use crate::utils::{into_lsp_error, panic_to_lsp_error};
use crate::{handlers, requests};
use futures::future::ready;
use futures::FutureExt;
use rome_console::markup;
use rome_service::workspace::{RageEntry, RageParams, RageResult};
use rome_service::{workspace, Workspace};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::Notify;
use tokio::task::spawn_blocking;
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::{lsp_types::*, ClientSocket};
use tower_lsp::{LanguageServer, LspService, Server};
use tracing::{error, info, trace, trace_span};

pub struct LSPServer {
    session: SessionHandle,
    sessions: Sessions,
}

impl LSPServer {
    fn new(session: SessionHandle, sessions: Sessions) -> Self {
        Self { session, sessions }
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

    async fn rage(&self, params: RageParams) -> LspResult<RageResult> {
        trace_span!("rome/rage", params=?params);

        let mut entries = vec![
            RageEntry::section("Server"),
            RageEntry::pair("Version", rome_service::VERSION),
            RageEntry::pair("Name", env!("CARGO_PKG_NAME")),
            RageEntry::pair("CPU Architecture", std::env::consts::ARCH),
            RageEntry::pair("OS", std::env::consts::OS),
        ];

        let RageResult {
            entries: workspace_entries,
        } = self.session.failsafe_rage(params);

        entries.extend(workspace_entries);

        if let Ok(sessions) = self.sessions.lock() {
            if sessions.len() > 1 {
                entries.push(RageEntry::markup(
                    markup!("\n"<Underline><Emphasis>"Other Active Server Workspaces:"</Emphasis></Underline>"\n"),
                ));

                for (key, session) in sessions.iter() {
                    if &self.session.key == key {
                        // Already printed above
                        continue;
                    }

                    let RageResult {
                        entries: workspace_entries,
                    } = session.failsafe_rage(params);

                    entries.extend(workspace_entries);

                    if let Ok(client_information) = session.client_information.lock() {
                        if let Some(information) = client_information.as_ref() {
                            entries.push(RageEntry::pair("Client Name", &information.name));

                            if let Some(version) = &information.version {
                                entries.push(RageEntry::pair("Client Version", version))
                            }
                        }
                    }
                }
            }
        }

        Ok(RageResult { entries })
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

        if let Some(client_info) = params.client_info {
            let mut client_information = self.session.client_information.lock().unwrap();
            *client_information = Some(ClientInformation {
                name: client_info.name,
                version: client_info.version,
            })
        };

        let init = InitializeResult {
            capabilities: server_capabilities(),
            server_info: Some(ServerInfo {
                name: String::from(env!("CARGO_PKG_NAME")),
                version: Some(rome_service::VERSION.to_string()),
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

impl Drop for LSPServer {
    fn drop(&mut self) {
        let mut sessions = self.sessions.lock().unwrap();

        let _removed = sessions.remove(&self.session.key);

        debug_assert!(_removed.is_some(), "Session did not exist.");
    }
}

type Sessions = Arc<Mutex<HashMap<SessionKey, SessionHandle>>>;

/// Factory data structure responsible for creating [ServerConnection] handles
/// for each incoming connection accepted by the server
#[derive(Default)]
pub struct ServerFactory {
    /// Synchronisation primitive used to broadcast a shutdown signal to all
    /// active connections
    cancellation: Arc<Notify>,
    /// Optional [Workspace] instance shared between all clients. Currently
    /// this field is always [None] (meaning each connection will get its own
    /// workspace) until we figure out how to handle concurrent access to the
    /// same workspace from multiple client
    workspace: Option<Arc<dyn Workspace>>,

    sessions: Sessions,

    session_id: AtomicUsize,
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
                    // The type of `result` is `Result<Result<R, RomeError>, JoinError>`,
                    // where the inner result is the return value of `$method` while the
                    // outer one is added by `spawn_blocking` to catch panics or
                    // cancellations of the task
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

        let session_key = SessionKey(self.session_id.fetch_add(1, Ordering::Relaxed));

        let mut builder = LspService::build(move |client| {
            let session = Session::new(session_key, client, workspace, self.cancellation.clone());
            let handle = Arc::new(session);

            let mut sessions = self.sessions.lock().unwrap();
            sessions.insert(session_key, handle.clone());
            LSPServer::new(handle, self.sessions.clone())
        });

        builder = builder.custom_method(SYNTAX_TREE_REQUEST, LSPServer::syntax_tree_request);

        // "shutdown" is not part of the Workspace API
        builder = builder.custom_method("rome/shutdown", |server: &LSPServer, (): ()| {
            tracing::info!("Sending shutdown signal");
            server.session.broadcast_shutdown();
            ready(Ok(Some(())))
        });

        builder = builder.custom_method("rome/rage", LSPServer::rage);

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
