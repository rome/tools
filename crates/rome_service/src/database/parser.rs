use std::any::type_name;

use rome_diagnostics::{Diagnostic, Severity};
use rome_fs::RomePath;
use rome_rowan::{AstNode, Language as RowanLanguage, SendNode, SyntaxNode};
use salsa::query_group;

use crate::{database::Inputs, RomeError};

/// Language-independent cache entry for a parsed file
///
/// This struct holds a handle to the root node of the parsed syntax tree,
/// along with the list of diagnostics emitted by the parser while generating
/// this entry.
///
/// It can be dynamically downcast into a concrete [SyntaxNode] or [AstNode] of
/// the corresponding language, generally through a language-specific capability
#[derive(Clone, Debug)]
pub(crate) struct AnyParse {
    root: SendNode,
}

impl AnyParse {
    pub(crate) fn new(root: SendNode) -> Self {
        Self { root }
    }

    pub(crate) fn syntax<L>(&self) -> SyntaxNode<L>
    where
        L: RowanLanguage + 'static,
    {
        self.root.clone().into_node().unwrap_or_else(|| {
            panic!(
                "could not downcast root node to language {}",
                type_name::<L>()
            )
        })
    }

    pub(crate) fn tree<N>(&self) -> N
    where
        N: AstNode,
        N::Language: 'static,
    {
        N::unwrap_cast(self.syntax::<N::Language>())
    }
}

impl PartialEq for AnyParse {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl Eq for AnyParse {}

#[query_group(ParserStorage)]
pub(crate) trait Parser: Inputs {
    fn parse(&self, name: RomePath) -> Result<(AnyParse, Vec<Diagnostic>), RomeError>;
    fn debug_print(&self, name: RomePath) -> Result<String, RomeError>;

    fn syntax(&self, name: RomePath) -> Result<AnyParse, RomeError>;
    fn diagnostics(&self, name: RomePath) -> Result<Vec<Diagnostic>, RomeError>;
    fn has_errors(&self, name: RomePath) -> bool;
}

fn parse(db: &dyn Parser, name: RomePath) -> Result<(AnyParse, Vec<Diagnostic>), RomeError> {
    let features = db.language_features(());
    let document = db.document(name.clone());

    let capabilities = features.get_capabilities(&name);
    let parser = capabilities
        .parse
        .ok_or_else(|| RomeError::SourceFileNotSupported(name.clone()))?;

    Ok(parser(&name, &document.content))
}

fn debug_print(db: &dyn Parser, name: RomePath) -> Result<String, RomeError> {
    let features = db.language_features(());
    let parse = db.syntax(name.clone())?;

    let capabilities = features.get_capabilities(&name);
    let printer = capabilities
        .debug_print
        .ok_or_else(|| RomeError::SourceFileNotSupported(name.clone()))?;

    Ok(printer(&name, parse))
}

fn syntax(db: &dyn Parser, name: RomePath) -> Result<AnyParse, RomeError> {
    let (syntax, _) = db.parse(name)?;
    Ok(syntax)
}

fn diagnostics(db: &dyn Parser, name: RomePath) -> Result<Vec<Diagnostic>, RomeError> {
    let (_, diagnostics) = db.parse(name)?;
    Ok(diagnostics)
}

fn has_errors(db: &dyn Parser, name: RomePath) -> bool {
    match db.diagnostics(name) {
        Ok(diagnostics) => diagnostics
            .iter()
            .any(|diag| diag.severity >= Severity::Error),
        Err(_) => true,
    }
}
