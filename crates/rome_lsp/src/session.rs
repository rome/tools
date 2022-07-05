use crate::config::Config;
use crate::config::CONFIGURATION_SECTION;

use crate::documents::Document;
use crate::url_interner::UrlInterner;
use crate::utils;
use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;
use rome_analyze::RuleCategories;
use rome_diagnostics::file::FileId;
use rome_fs::RomePath;
use rome_service::workspace;
use rome_service::workspace::PullDiagnosticsParams;
use rome_service::workspace::UpdateSettingsParams;
use rome_service::RomeError;
use rome_service::Workspace;
use std::collections::HashMap;
use std::sync::RwLock;
use tower_lsp::lsp_types;
use tracing::{error, trace};

/// Represents the state of an LSP server session.
pub(crate) struct Session {
    /// The LSP client for this session.
    pub(crate) client: tower_lsp::Client,
    /// The capabilities provided by the client as part of [`lsp_types::InitializeParams`]
    pub(crate) client_capabilities: RwLock<Option<lsp_types::ClientCapabilities>>,

    /// the configuration of the LSP
    pub(crate) config: RwLock<Config>,

    pub(crate) workspace: Box<dyn Workspace>,
    documents: RwLock<HashMap<lsp_types::Url, Document>>,
    url_interner: RwLock<UrlInterner>,
}

impl Session {
    pub(crate) fn new(client: tower_lsp::Client) -> Self {
        let client_capabilities = RwLock::new(Default::default());
        let documents = Default::default();
        let url_interner = Default::default();
        let config = RwLock::new(Config::new());
        Self {
            client,
            client_capabilities,
            workspace: workspace::server(),
            documents,
            url_interner,
            config,
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
    pub(crate) async fn update_diagnostics(&self, url: lsp_types::Url) -> anyhow::Result<()> {
        let rome_path = self.file_path(&url);
        let doc = self.document(&url)?;

        let workspace_settings = self.config.read().unwrap().get_workspace_settings();

        let diagnostics = if workspace_settings.analysis.enable_diagnostics {
            let diagnostics = self.workspace.pull_diagnostics(PullDiagnosticsParams {
                path: rome_path,
                categories: RuleCategories::SYNTAX | RuleCategories::LINT,
            })?;

            diagnostics
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

    /// Checks `analysis.enable_diagnostics` in this session's workspace settings`
    pub(crate) fn diagnostics_enabled(&self) -> bool {
        self.config
            .read()
            .unwrap()
            .get_workspace_settings()
            .analysis
            .enable_diagnostics
    }

    /// Requests "workspace/configuration" from client and updates Session config
    pub(crate) async fn fetch_client_configuration(&self) {
        let item = lsp_types::ConfigurationItem {
            scope_uri: None,
            section: Some(String::from(CONFIGURATION_SECTION)),
        };
        let items = vec![item];
        let configurations = self.client.configuration(items).await;

        if let Ok(configurations) = configurations {
            configurations.into_iter().next().and_then(|configuration| {
                let mut config = self.config.write().unwrap();

                config
                    .set_workspace_settings(configuration)
                    .map_err(|err| {
                        error!("Cannot set workspace settings: {}", err);
                    })
                    .ok()?;

                self.workspace
                    .update_settings(UpdateSettingsParams {
                        settings: config.as_workspace_settings(),
                    })
                    .ok()?;

                Some(())
            });
        } else {
            trace!("Cannot read configuration from the client");
        }
    }
}
