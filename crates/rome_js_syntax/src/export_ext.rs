use rome_rowan::{AstNode, SyntaxResult};

use crate::{
    AnyJsExportNamedSpecifier, JsExportNamedClause, JsExportNamedSpecifierList,
    JsReferenceIdentifier, JsSyntaxToken,
};

impl AnyJsExportNamedSpecifier {
    pub fn type_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsExportNamedSpecifier::JsExportNamedShorthandSpecifier(specifier) => {
                specifier.type_token()
            }
            AnyJsExportNamedSpecifier::JsExportNamedSpecifier(specifier) => specifier.type_token(),
        }
    }

    /// Returns the local name of the export.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_syntax::{AnyJsExportNamedSpecifier, T};
    /// use rome_js_factory::make;
    ///
    /// let specifier = make::js_export_named_shorthand_specifier(
    ///     make::js_reference_identifier(make::ident("a"))
    /// ).with_type_token(make::token(T![type])).build();
    /// let export = AnyJsExportNamedSpecifier::from(specifier.clone());
    ///
    /// assert_eq!(export.local_name(), specifier.name());
    ///
    /// let specifier = make::js_export_named_specifier(
    ///     make::js_reference_identifier(make::ident("a")),
    ///     make::token(T![as]),
    ///     make::js_literal_export_name(make::ident("b")),
    /// ).build();
    /// let export = AnyJsExportNamedSpecifier::from(specifier.clone());
    ///
    /// assert_eq!(export.local_name(), specifier.local_name());
    /// ```
    pub fn local_name(&self) -> SyntaxResult<JsReferenceIdentifier> {
        match self {
            AnyJsExportNamedSpecifier::JsExportNamedShorthandSpecifier(specifier) => {
                specifier.name()
            }
            AnyJsExportNamedSpecifier::JsExportNamedSpecifier(specifier) => specifier.local_name(),
        }
    }

    /// Returns `true` if this export is an inline type export or an export part of an `export type`.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_syntax::{AnyJsExportNamedSpecifier, T};
    /// use rome_js_factory::make;
    ///
    /// let specifier = make::js_export_named_shorthand_specifier(
    ///     make::js_reference_identifier(make::ident("a"))
    /// ).with_type_token(make::token(T![type])).build();
    /// let export = AnyJsExportNamedSpecifier::from(specifier);
    ///
    /// assert!(export.is_type_only());
    /// ```
    pub fn is_type_only(&self) -> bool {
        if self.type_token().is_some() {
            return true;
        }
        if let Some(export) = self.parent::<JsExportNamedSpecifierList>() {
            if let Some(export) = export.parent::<JsExportNamedClause>() {
                if export.type_token().is_some() {
                    return true;
                }
            }
        }
        false
    }

    pub fn with_type_token(self, type_token: Option<JsSyntaxToken>) -> Self {
        match self {
            AnyJsExportNamedSpecifier::JsExportNamedShorthandSpecifier(specifier) => {
                specifier.with_type_token(type_token).into()
            }
            AnyJsExportNamedSpecifier::JsExportNamedSpecifier(specifier) => {
                specifier.with_type_token(type_token).into()
            }
        }
    }
}
