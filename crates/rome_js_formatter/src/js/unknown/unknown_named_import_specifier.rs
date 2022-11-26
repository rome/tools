use crate::FormatUnknownNodeRule;
use rome_js_syntax::JsUnknownNamedImportSpecifier;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsUnknownNamedImportSpecifier;

impl FormatUnknownNodeRule<JsUnknownNamedImportSpecifier> for FormatJsUnknownNamedImportSpecifier {}
