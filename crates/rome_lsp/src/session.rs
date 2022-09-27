use crate::config::Config;
use crate::config::CONFIGURATION_SECTION;
use crate::documents::Document;
use crate::url_interner::UrlInterner;
use crate::utils;
use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;
use rome_analyze::RuleCategories;
use rome_console::markup;
use rome_diagnostics::file::FileId;
use rome_fs::{FileSystem, OsFileSystem, RomePath};
use rome_service::configuration::Configuration;
use rome_service::workspace::{FeatureName, PullDiagnosticsParams, SupportsFeatureParams};
use rome_service::workspace::{RageEntry, RageParams, RageResult, UpdateSettingsParams};
use rome_service::{load_config, Workspace};
use rome_service::{DynRef, RomeError};
use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::Url;
use tracing::{error, info, trace};

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
    /// The capabilities provided by the client as part of [`lsp_types::InitializeParams`]
    pub(crate) client_capabilities: RwLock<Option<lsp_types::ClientCapabilities>>,

    pub(crate) client_information: Mutex<Option<ClientInformation>>,

    /// the configuration of the LSP
    pub(crate) config: RwLock<Config>,

    pub(crate) workspace: Arc<dyn Workspace>,

    /// File system to read files inside the workspace
    pub(crate) fs: DynRef<'static, dyn FileSystem>,

    /// The configuration coming from `rome.json` file
    pub(crate) configuration: RwLock<Option<Configuration>>,

    pub(crate) root_uri: RwLock<Option<Url>>,

    documents: RwLock<HashMap<lsp_types::Url, Document>>,
    url_interner: RwLock<UrlInterner>,

    cancellation: Arc<Notify>,
}

pub(crate) type SessionHandle = Arc<Session>;

impl Session {
    pub(crate) fn new(
        key: SessionKey,
        client: tower_lsp::Client,
        workspace: Arc<dyn Workspace>,
        cancellation: Arc<Notify>,
    ) -> Self {
        let client_capabilities = RwLock::new(Default::default());
        let documents = Default::default();
        let url_interner = Default::default();
        let config = RwLock::new(Config::new());
        let configuration = RwLock::new(None);
        let root_uri = RwLock::new(None);
        Self {
            key,
            client,
            client_information: Default::default(),
            client_capabilities,
            workspace,
            documents,
            url_interner,
            config,
            fs: DynRef::Owned(Box::new(OsFileSystem)),
            configuration,
            root_uri,
            cancellation,
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
    #[tracing::instrument(level = "trace", skip(self), err)]
    pub(crate) async fn update_diagnostics(&self, url: lsp_types::Url) -> anyhow::Result<()> {
        let rome_path = self.file_path(&url);
        let doc = self.document(&url)?;
        let unsupported_lint = self.workspace.supports_feature(SupportsFeatureParams {
            feature: FeatureName::Lint,
            path: rome_path.clone(),
        })?;

        let diagnostics = if unsupported_lint.reason.is_none() {
            let result = self.workspace.pull_diagnostics(PullDiagnosticsParams {
                path: rome_path,
                categories: RuleCategories::SYNTAX | RuleCategories::LINT,
                max_diagnostics: u64::MAX,
            })?;

            result
                .diagnostics
                .into_iter()
                .filter_map(|d| utils::diagnostic_to_lsp(d, &url, &doc.line_index))
                .collect()
        } else {
            // Sending empty vector clears published diagnostics
            vec![]
        };

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
        self.client_capabilities
            .read()
            .unwrap()
            .as_ref()
            .and_then(|c| c.workspace.as_ref())
            .and_then(|c| c.did_change_configuration)
            .and_then(|c| c.dynamic_registration)
            == Some(true)
    }

    /// This function attempts to read the configuration from the root URI
    pub(crate) async fn update_configuration(&self) {
        let root_uri = self.root_uri.read().unwrap();
        let base_path =  root_uri.as_ref().and_then(|root_uri| match root_uri.to_file_path() {
            Ok(base_path) => Some(base_path),
            Err(()) => {
                error!("The Workspace root URI {root_uri:?} could not be parsed as a filesystem path");
                None
            }
        });

        match load_config(&self.fs, base_path) {
            Ok(Some(configuration)) => {
                info!("Configuration found, and it is valid!");
                self.configuration.write().unwrap().replace(configuration);
            }
            Err(err) => {
                error!("Couldn't load the configuration file, reason:\n {}", err);
            }
            _ => {}
        };
    }

    /// Requests "workspace/configuration" from client and updates Session config
    pub(crate) async fn fetch_client_configuration(&self) {
        let item = lsp_types::ConfigurationItem {
            scope_uri: None,
            section: Some(String::from(CONFIGURATION_SECTION)),
        };
        let items = vec![item];
        let client_configurations = self.client.configuration(items).await;

        if let Ok(client_configurations) = client_configurations {
            client_configurations
                .into_iter()
                .next()
                .and_then(|client_configuration| {
                    let mut config = self.config.write().unwrap();

                    config
                        .set_workspace_settings(client_configuration)
                        .map_err(|err| {
                            error!("Cannot set workspace settings: {}", err);
                        })
                        .ok()?;
                    self.update_workspace_settings();

                    Some(())
                });
        } else {
            trace!("Cannot read configuration from the client");
        }
    }

    /// If updates the [Workspace] settings with the new configuration that was
    /// read from file.
    #[tracing::instrument(level = "debug", skip(self))]
    pub(crate) fn update_workspace_settings(&self) {
        let mut configuration = self.configuration.write().unwrap();

        // This operation is intended, we want to consume the configuration because once it's read
        // from the LSP, it's not needed anymore
        if let Some(configuration) = configuration.take() {
            trace!(
                "The LSP will now use the following configuration: \n {:?}",
                &configuration
            );

            let result = self
                .workspace
                .update_settings(UpdateSettingsParams { configuration });

            if let Err(error) = result {
                error!("{:?}", &error)
            }
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
