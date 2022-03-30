use crate::documents::Document;
use anyhow::Result;
use lspower::lsp::TextDocumentIdentifier;
use rome_js_parser::parse;
use serde::{Deserialize, Serialize};
use tracing::{info, trace};

pub const SYNTAX_TREE_REQUEST: &str = "rome/syntaxTree";

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyntaxTreePayload {
    pub text_document: TextDocumentIdentifier,
}

pub fn syntax_tree(document: Document) -> Result<String> {
    info!("Showing syntax tree");
    trace!("Showing syntax tree for: {:?}", document);
    let text = &document.text;
    let file_id = document.file_id();
    let source_type = document.get_source_type();
    let parse_result = parse(text, file_id, source_type);
    let cst = format!("{:#?}", parse_result.tree());

    Ok(cst)
}
