use crate::{JsAnyImportClause, JsImport, JsModuleSource, TextSize};
use rome_rowan::{SyntaxResult, SyntaxTokenText};

impl JsImport {
    /// It checks if the source of an import against the string `source_to_check`
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::make::{js_reference_identifier, ident, js_module_source, js_import_default_clause, token, js_identifier_binding, js_import};
    /// use rome_js_syntax::{JsAnyBinding, JsAnyImportClause, T};
    /// let source = js_module_source(ident("react"));
    /// let binding = js_identifier_binding(ident("React"));
    /// let clause = js_import_default_clause(JsAnyBinding::JsIdentifierBinding(binding), token(T![from]), source).build();
    /// let import = js_import(token(T![import]), JsAnyImportClause::JsImportDefaultClause(clause)).build();
    /// assert_eq!(import.source_is("react"), Ok(true));
    /// assert_eq!(import.source_is("React"), Ok(false));
    /// ```
    pub fn source_is(&self, source_to_check: &str) -> SyntaxResult<bool> {
        let clause = self.import_clause()?;
        let source = match clause {
            JsAnyImportClause::JsImportBareClause(node) => node.source(),
            JsAnyImportClause::JsImportDefaultClause(node) => node.source(),
            JsAnyImportClause::JsImportNamedClause(node) => node.source(),
            JsAnyImportClause::JsImportNamespaceClause(node) => node.source(),
        }?;

        Ok(source.inner_string_text()?.text() == source_to_check)
    }
}

impl JsModuleSource {
    /// Get the inner text of a string not including the quotes
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::make::{ident, js_module_source};
    /// use rome_js_syntax::{JsAnyBinding, JsAnyImportClause, T};
    /// let source = js_module_source(ident("react"));
    /// let text = source.inner_string_text().unwrap();
    /// assert_eq!(text.text(), "react");
    /// ```
    pub fn inner_string_text(&self) -> SyntaxResult<SyntaxTokenText> {
        let value = self.value_token()?;
        let mut text = value.token_text();

        static QUOTES: [char; 2] = ['"', '\''];

        if text.starts_with(QUOTES) {
            let range = text.range().add_start(TextSize::from(1));
            text = text.slice(range);
        }

        if text.ends_with(QUOTES) {
            let range = text.range().sub_end(TextSize::from(1));
            text = text.slice(range);
        }

        Ok(text)
    }
}
