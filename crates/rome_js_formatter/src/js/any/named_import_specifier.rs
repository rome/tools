//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyNamedImportSpecifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyNamedImportSpecifier;
impl FormatRule<JsAnyNamedImportSpecifier> for FormatJsAnyNamedImportSpecifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyNamedImportSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(node) => {
                node.format().fmt(f)
            }
            JsAnyNamedImportSpecifier::JsNamedImportSpecifier(node) => node.format().fmt(f),
            JsAnyNamedImportSpecifier::JsBogusNamedImportSpecifier(node) => node.format().fmt(f),
        }
    }
}
