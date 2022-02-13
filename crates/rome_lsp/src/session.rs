use std::{collections::HashMap, error::Error, fmt::Display, sync::Arc};

use lspower::jsonrpc::Error as LspError;
use lspower::lsp;
use parking_lot::{Mutex, RwLock};
use rome_analyze::AnalysisServer;

use crate::{
    capabilities::server_capabilities, documents::Document, handlers, url_interner::UrlInterner,
};

pub struct Session {
    pub client: lspower::Client,
    pub server_capabilities: RwLock<lsp::ServerCapabilities>,
    pub client_capabilities: RwLock<Option<lsp::ClientCapabilities>>,
    pub documents: RwLock<HashMap<lsp::Url, Document>>,
    pub analysis_server: Arc<Mutex<AnalysisServer>>,
    url_interner: RwLock<UrlInterner>,
}

#[derive(Debug)]
pub enum SessionError {
    DocumentNotFound { url: lsp::Url },
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

pub fn into_lsp_error(err: impl Display) -> LspError {
    let mut error = LspError::internal_error();
    error.data = Some(err.to_string().into());
    error
}

impl Session {
    pub fn new(client: lspower::Client) -> Self {
        let server_capabilities = RwLock::new(server_capabilities());
        let client_capabilities = RwLock::new(Default::default());
        let documents = Default::default();
        let analysis_server = Default::default();
        let url_interner = Default::default();
        Self {
            client,
            server_capabilities,
            client_capabilities,
            documents,
            analysis_server,
            url_interner,
        }
    }

    pub fn document(&self, url: &lsp::Url) -> Result<Document, SessionError> {
        self.documents
            .read()
            .get(url)
            .cloned()
            .ok_or_else(|| SessionError::DocumentNotFound {
                url: url.to_owned(),
            })
    }

    pub fn insert_document(&self, url: lsp::Url, document: Document) {
        self.documents.write().insert(url, document);
    }

    pub fn remove_document(&self, url: &lsp::Url) {
        self.documents.write().remove(url);
    }

    pub fn document_version(&self, url: &lsp::Url) -> Result<i32, SessionError> {
        self.document(url).map(|d| d.version)
    }

    pub fn file_id(&self, url: lsp::Url) -> usize {
        self.url_interner.write().intern(url)
    }

    #[allow(unused)]
    pub fn url(&self, file_id: usize) -> lsp::Url {
        self.url_interner.read().lookup(file_id).to_owned()
    }

    pub async fn update_diagnostics(&self, url: lsp::Url) -> anyhow::Result<()> {
        let doc = self.document(&url)?;

        let mut analysis_server = AnalysisServer::default();
        analysis_server.set_file_text(doc.file_id, doc.text);

        let handle = tokio::task::spawn_blocking(move || {
            handlers::analysis::diagnostics(analysis_server, doc.file_id)
        });

        let diagnostics = handle.await??;

        let version = self.document_version(&url)?;

        if version == doc.version {
            self.client
                .publish_diagnostics(url, diagnostics, Some(doc.version))
                .await;
        }
        Ok(())
    }
}
