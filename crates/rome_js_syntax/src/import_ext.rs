use crate::{
    inner_string_text, AnyJsBinding, AnyJsImportClause, AnyJsNamedImportSpecifier, JsImport,
    JsImportNamedClause, JsModuleSource, JsNamedImportSpecifierList, JsNamedImportSpecifiers,
    JsSyntaxToken,
};
use rome_rowan::{AstNode, SyntaxResult, TokenText};

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

        Ok(source.inner_string_text()?.text() == source_to_check)
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
    /// let text = source.inner_string_text().unwrap();
    /// assert_eq!(text.text(), "react");
    /// ```
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}

impl AnyJsImportClause {
    pub fn type_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsImportClause::JsImportBareClause(_) => None,
            AnyJsImportClause::JsImportDefaultClause(clause) => clause.type_token(),
            AnyJsImportClause::JsImportNamedClause(clause) => clause.type_token(),
            AnyJsImportClause::JsImportNamespaceClause(clause) => clause.type_token(),
        }
    }
}

impl AnyJsNamedImportSpecifier {
    pub fn type_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => None,
            AnyJsNamedImportSpecifier::JsNamedImportSpecifier(specifier) => specifier.type_token(),
            AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(specifier) => {
                specifier.type_token()
            }
        }
    }

    pub fn local_name(&self) -> Option<AnyJsBinding> {
        match self {
            AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => None,
            AnyJsNamedImportSpecifier::JsNamedImportSpecifier(specifier) => {
                specifier.local_name().ok()
            }
            AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(specifier) => {
                specifier.local_name().ok()
            }
        }
    }

    /// Returns `true` if this import is an inline type import or an import part of an `import type`.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_syntax::{AnyJsNamedImportSpecifier, T};
    /// use rome_js_factory::make;
    ///
    /// let specifier = make::js_shorthand_named_import_specifier(
    ///     make::js_identifier_binding(make::ident("a")).into()
    /// ).with_type_token(make::token(T![type])).build();
    /// let export = AnyJsNamedImportSpecifier::from(specifier);
    ///
    /// assert!(export.is_type_only());
    /// ```
    pub fn is_type_only(&self) -> bool {
        if self.type_token().is_some() {
            return true;
        }
        if let Some(import) = self.parent::<JsNamedImportSpecifierList>() {
            if let Some(import) = import.parent::<JsNamedImportSpecifiers>() {
                if let Some(import) = import.parent::<JsImportNamedClause>() {
                    if import.type_token().is_some() {
                        return true;
                    }
                }
            }
        }
        false
    }
}
