use lspower::lsp::{
	CodeActionProviderCapability, OneOf, ServerCapabilities, TextDocumentSyncCapability,
	TextDocumentSyncKind,
};

pub(crate) fn server_capabilities() -> ServerCapabilities {
	ServerCapabilities {
		text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
		code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
		document_formatting_provider: Some(OneOf::Left(true)),
		..Default::default()
	}
}
