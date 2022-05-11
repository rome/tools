use crate::config::Config;
use crate::config::CONFIGURATION_SECTION;

use crate::{documents::Document, handlers, url_interner::UrlInterner};
use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;
use parking_lot::RwLock;
use rome_diagnostics::file::FileId;
use std::{collections::HashMap, error::Error, fmt::Display};
use tower_lsp::jsonrpc::Error as LspError;
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

    documents: RwLock<HashMap<lsp_types::Url, Document>>,
    url_interner: RwLock<UrlInterner>,
}

#[derive(Debug)]
pub(crate) enum SessionError {
    DocumentNotFound { url: lsp_types::Url },
}

impl Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::DocumentNotFound { url } => {
                write!(f, "Document not found: {}", url)
            }
        }
    }
}

impl Error for SessionError {}

impl From<SessionError> for LspError {
    fn from(err: SessionError) -> Self {
        match err {
            SessionError::DocumentNotFound { .. } => {
                let mut error = LspError::internal_error();
                error.data = Some(err.to_string().into());
                error
            }
        }
    }
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
            documents,
            url_interner,
            config,
        }
    }

    /// Get a [`Document`] matching the provided [`lsp_types::Url`]
    ///
    /// If document does not exist, result is [SessionError::DocumentNotFound]
    pub(crate) fn document(&self, url: &lsp_types::Url) -> Result<Document, SessionError> {
        self.documents
            .read()
            .get(url)
            .cloned()
            .ok_or_else(|| SessionError::DocumentNotFound {
                url: url.to_owned(),
            })
    }

    /// Set the [`Document`] for the provided [`lsp_types::Url`]
    ///
    /// Used by [`handlers::text_document] to synchronize documents with the client.
    pub(crate) fn insert_document(&self, url: lsp_types::Url, document: Document) {
        self.documents.write().insert(url, document);
    }

    /// Remove the [`Document`] matching the provided [`lsp_types::Url`]
    pub(crate) fn remove_document(&self, url: &lsp_types::Url) {
        self.documents.write().remove(url);
    }

    /// Get the version for [`Document`] matching the url. Should increase with every edit.
    pub(crate) fn document_version(&self, url: &lsp_types::Url) -> Result<i32, SessionError> {
        self.document(url).map(|d| d.version)
    }

    /// Return the unique [FileId] associated with the url for this [Session].
    /// This will assign a new FileId if there isn't one for the provided url.
    pub(crate) fn file_id(&self, url: lsp_types::Url) -> FileId {
        self.url_interner.write().intern(url)
    }

    /// Computes diagnostics for the file matching the provided url and publishes
    /// them to the client. Called from [`handlers::text_document`] when a file's
    /// contents changes.
    pub(crate) async fn update_diagnostics(&self, url: lsp_types::Url) -> anyhow::Result<()> {
        let doc = self.document(&url)?;

        let workspace_settings = self.config.read().get_workspace_settings();

        let diagnostics = if workspace_settings.analysis.enable_diagnostics {
            let file_id = doc.file_id();

            let handle = tokio::task::spawn_blocking(move || {
                handlers::analysis::diagnostics(file_id, &doc.text)
            });

            handle.await??
        } else {
            // Sending empty vector clears published diagnostics
            vec![]
        };

        let version = self.document_version(&url)?;

        if version == doc.version {
            self.client
                .publish_diagnostics(url, diagnostics, Some(doc.version))
                .await;
        }
        Ok(())
    }

    /// Updates diagnostics for every [`Document`] in this [`Session`]
    pub(crate) async fn update_all_diagnostics(&self) {
        let mut futures: FuturesUnordered<_> = self
            .documents
            .read()
            .keys()
            .cloned()
            .map(|url| self.update_diagnostics(url))
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
                self.config
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
    }
}
