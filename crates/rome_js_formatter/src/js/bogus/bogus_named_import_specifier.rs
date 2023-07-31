use crate::FormatBogusNodeRule;
use rome_js_syntax::JsBogusNamedImportSpecifier;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusNamedImportSpecifier;

impl FormatBogusNodeRule<JsBogusNamedImportSpecifier> for FormatJsBogusNamedImportSpecifier {}
