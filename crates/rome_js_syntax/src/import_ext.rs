use crate::{inner_text, AnyJsImportClause, JsImport, JsModuleSource};
use rome_rowan::{SyntaxResult, SyntaxTokenText};

impl JsImport {
    /// It checks if the source of an import against the string `source_to_check`
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::make::{js_reference_identifier, ident, js_module_source, js_import_default_clause, token, js_identifier_binding, js_import};
    /// use rome_js_syntax::{AnyJsBinding, AnyJsImportClause, T};
    /// let source = js_module_source(ident("react"));
    /// let binding = js_identifier_binding(ident("React"));
    /// let clause = js_import_default_clause(AnyJsBinding::JsIdentifierBinding(binding), token(T![from]), source).build();
    /// let import = js_import(token(T![import]), AnyJsImportClause::JsImportDefaultClause(clause)).build();
    /// assert_eq!(import.source_is("react"), Ok(true));
    /// assert_eq!(import.source_is("React"), Ok(false));
    /// ```
    pub fn source_is(&self, source_to_check: &str) -> SyntaxResult<bool> {
        let clause = self.import_clause()?;
        let source = match clause {
            AnyJsImportClause::JsImportBareClause(node) => node.source(),
            AnyJsImportClause::JsImportDefaultClause(node) => node.source(),
            AnyJsImportClause::JsImportNamedClause(node) => node.source(),
            AnyJsImportClause::JsImportNamespaceClause(node) => node.source(),
        }?;

        Ok(source.inner_text()?.text() == source_to_check)
    }
}

impl JsModuleSource {
    /// Get the inner text of a string not including the quotes
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::make::{ident, js_module_source};
    /// use rome_js_syntax::{AnyJsBinding, AnyJsImportClause, T};
    /// use rome_rowan::TriviaPieceKind;
    /// let source = js_module_source(ident("react").with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]));
    /// let text = source.inner_text().unwrap();
    /// assert_eq!(text.text(), "react");
    /// ```
    pub fn inner_text(&self) -> SyntaxResult<SyntaxTokenText> {
        Ok(inner_text(&self.value_token()?))
    }
}
