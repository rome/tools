use crate::config::Config;
use crate::config::CONFIGURATION_SECTION;
use crate::documents::Document;
use crate::url_interner::UrlInterner;
use crate::utils;
use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;
use rome_analyze::RuleCategories;
use rome_console::markup;
use rome_diagnostics::location::FileId;
use rome_fs::{FileSystem, OsFileSystem, RomePath};
use rome_service::workspace::{FeatureName, PullDiagnosticsParams, SupportsFeatureParams};
use rome_service::workspace::{RageEntry, RageParams, RageResult, UpdateSettingsParams};
use rome_service::{load_config, Workspace};
use rome_service::{DynRef, RomeError};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::sync::Notify;
use tokio::sync::OnceCell;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::Url;
use tracing::{error, info};

pub(crate) struct ClientInformation {
    /// The name of the client
    pub(crate) name: String,

    /// The version of the client
    pub(crate) version: Option<String>,
}

/// Key, uniquely identifying a LSP session.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub(crate) struct SessionKey(pub u64);

/// Represents the state of an LSP server session.
pub(crate) struct Session {
    /// The unique key identifying this session.
    pub(crate) key: SessionKey,

    /// The LSP client for this session.
    pub(crate) client: tower_lsp::Client,

    /// The parameters provided by the client in the "initialize" request
    initialize_params: OnceCell<InitializeParams>,

    /// the configuration of the LSP
    pub(crate) config: RwLock<Config>,

    pub(crate) workspace: Arc<dyn Workspace>,

    /// File system to read files inside the workspace
    pub(crate) fs: DynRef<'static, dyn FileSystem>,

    documents: RwLock<HashMap<lsp_types::Url, Document>>,
    url_interner: RwLock<UrlInterner>,

    pub(crate) cancellation: Arc<Notify>,
}

/// The parameters provided by the client in the "initialize" request
struct InitializeParams {
    /// The capabilities provided by the client as part of [`lsp_types::InitializeParams`]
    client_capabilities: lsp_types::ClientCapabilities,
    client_information: Option<ClientInformation>,
    root_uri: Option<Url>,
}

pub(crate) type SessionHandle = Arc<Session>;

impl Session {
    pub(crate) fn new(
        key: SessionKey,
        client: tower_lsp::Client,
        workspace: Arc<dyn Workspace>,
        cancellation: Arc<Notify>,
    ) -> Self {
        let documents = Default::default();
        let url_interner = Default::default();
        let config = RwLock::new(Config::new());
        Self {
            key,
            client,
            initialize_params: OnceCell::default(),
            workspace,
            documents,
            url_interner,
            config,
            fs: DynRef::Owned(Box::new(OsFileSystem)),
            cancellation,
        }
    }

    /// Initialize this session instance with the incoming initialization parameters from the client
    pub(crate) fn initialize(
        &self,
        client_capabilities: lsp_types::ClientCapabilities,
        client_information: Option<ClientInformation>,
        root_uri: Option<Url>,
    ) {
        let result = self.initialize_params.set(InitializeParams {
            client_capabilities,
            client_information,
            root_uri,
        });

        if let Err(err) = result {
            error!("Failed to initialize session: {err}");
        }
    }

    /// Get a [`Document`] matching the provided [`lsp_types::Url`]
    ///
    /// If document does not exist, result is [SessionError::DocumentNotFound]
    pub(crate) fn document(&self, url: &lsp_types::Url) -> Result<Document, RomeError> {
        self.documents
            .read()
            .unwrap()
            .get(url)
            .cloned()
            .ok_or(RomeError::NotFound)
    }

    /// Set the [`Document`] for the provided [`lsp_types::Url`]
    ///
    /// Used by [`handlers::text_document] to synchronize documents with the client.
    pub(crate) fn insert_document(&self, url: lsp_types::Url, document: Document) {
        self.documents.write().unwrap().insert(url, document);
    }

    /// Remove the [`Document`] matching the provided [`lsp_types::Url`]
    pub(crate) fn remove_document(&self, url: &lsp_types::Url) {
        self.documents.write().unwrap().remove(url);
    }

    /// Return the unique [FileId] associated with the url for this [Session].
    /// This will assign a new FileId if there isn't one for the provided url.
    pub(crate) fn file_id(&self, url: lsp_types::Url) -> FileId {
        self.url_interner.write().unwrap().intern(url)
    }

    pub(crate) fn file_path(&self, url: &lsp_types::Url) -> RomePath {
        let file_id = self.file_id(url.clone());
        RomePath::new(url.path(), file_id)
    }

    /// Computes diagnostics for the file matching the provided url and publishes
    /// them to the client. Called from [`handlers::text_document`] when a file's
    /// contents changes.
    #[tracing::instrument(level = "debug", skip_all, fields(url = display(&url), diagnostic_count), err)]
    pub(crate) async fn update_diagnostics(&self, url: lsp_types::Url) -> anyhow::Result<()> {
        let rome_path = self.file_path(&url);
        let doc = self.document(&url)?;
        let unsupported_lint = self.workspace.supports_feature(SupportsFeatureParams {
            feature: FeatureName::Lint,
            path: rome_path.clone(),
        })?;

        let diagnostics = if let Some(reason) = unsupported_lint.reason {
            tracing::trace!("linting not supported: {reason:?}");
            // Sending empty vector clears published diagnostics
            vec![]
        } else {
            let result = self.workspace.pull_diagnostics(PullDiagnosticsParams {
                path: rome_path,
                categories: RuleCategories::SYNTAX | RuleCategories::LINT,
                max_diagnostics: u64::MAX,
            })?;

            tracing::trace!("rome diagnostics: {:#?}", result.diagnostics);

            let result = result
                .diagnostics
                .into_iter()
                .filter_map(
                    |d| match utils::diagnostic_to_lsp(d, &url, &doc.line_index) {
                        Ok(diag) => Some(diag),
                        Err(err) => {
                            tracing::error!("failed to convert diagnostic to LSP: {err:?}");
                            None
                        }
                    },
                )
                .collect();

            tracing::trace!("lsp diagnostics: {:#?}", result);

            result
        };

        tracing::Span::current().record("diagnostic_count", diagnostics.len());

        self.client
            .publish_diagnostics(url, diagnostics, Some(doc.version))
            .await;

        Ok(())
    }

    /// Updates diagnostics for every [`Document`] in this [`Session`]
    pub(crate) async fn update_all_diagnostics(&self) {
        let mut futures: FuturesUnordered<_> = self
            .documents
            .read()
            .unwrap()
            .keys()
            .map(|url| self.update_diagnostics(url.clone()))
            .collect();

        while let Some(result) = futures.next().await {
            if let Err(e) = result {
                error!("Error while updating diagnostics: {}", e);
            }
        }
    }

    /// True if the client supports dynamic registration of "workspace/didChangeConfiguration" requests
    pub(crate) fn can_register_did_change_configuration(&self) -> bool {
        self.initialize_params
            .get()
            .and_then(|c| c.client_capabilities.workspace.as_ref())
            .and_then(|c| c.did_change_configuration)
            .and_then(|c| c.dynamic_registration)
            == Some(true)
    }

    /// Returns the base path of the workspace on the filesystem if it has one
    pub(crate) fn base_path(&self) -> Option<PathBuf> {
        let initialize_params = self.initialize_params.get()?;

        let root_uri = initialize_params.root_uri.as_ref()?;
        match root_uri.to_file_path() {
            Ok(base_path) => Some(base_path),
            Err(()) => {
                error!(
                    "The Workspace root URI {root_uri:?} could not be parsed as a filesystem path"
                );
                None
            }
        }
    }

    /// Returns a reference to the client informations for this session
    pub(crate) fn client_information(&self) -> Option<&ClientInformation> {
        self.initialize_params.get()?.client_information.as_ref()
    }

    /// This function attempts to read the `rome.json` configuration file from
    /// the root URI and update the workspace settings accordingly
    #[tracing::instrument(level = "debug", skip(self))]
    pub(crate) async fn load_workspace_settings(&self) {
        let base_path = self.base_path();

        match load_config(&self.fs, base_path) {
            Ok(Some(configuration)) => {
                info!("Loaded workspace settings: {configuration:#?}");

                let result = self
                    .workspace
                    .update_settings(UpdateSettingsParams { configuration });

                if let Err(error) = result {
                    error!("Failed to set workspace settings: {}", error)
                }
            }
            Ok(None) => {
                // Ignore, load_config already logs an error in this case
            }
            Err(err) => {
                error!("Couldn't load the workspace settings, reason:\n {}", err);
            }
        }
    }

    /// Requests "workspace/configuration" from client and updates Session config
    #[tracing::instrument(level = "debug", skip(self))]
    pub(crate) async fn load_client_configuration(&self) {
        let item = lsp_types::ConfigurationItem {
            scope_uri: None,
            section: Some(String::from(CONFIGURATION_SECTION)),
        };

        let client_configurations = match self.client.configuration(vec![item]).await {
            Ok(client_configurations) => client_configurations,
            Err(err) => {
                error!("Couldn't read configuration from the client: {err}");
                return;
            }
        };

        let client_configuration = client_configurations.into_iter().next();

        if let Some(client_configuration) = client_configuration {
            info!("Loaded client configuration: {client_configuration:#?}");

            let mut config = self.config.write().unwrap();
            if let Err(err) = config.set_workspace_settings(client_configuration) {
                error!("Couldn't set client configuration: {}", err);
            }
        } else {
            info!("Client did not return any configuration");
        }
    }

    /// Broadcast a shutdown signal to all active connections
    pub(crate) fn broadcast_shutdown(&self) {
        self.cancellation.notify_one();
    }

    pub(crate) fn failsafe_rage(&self, params: RageParams) -> RageResult {
        match self.workspace.rage(params) {
            Ok(result) => result,
            Err(err) => {
                let entries = vec![
                    RageEntry::section("Workspace"),
                    RageEntry::markup(markup! {
                        <Error>"\u{2716} Rage command failed:"</Error> {&format!("{err}")}
                    }),
                ];

                RageResult { entries }
            }
        }
    }
}
