//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyNamedImportSpecifier;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyNamedImportSpecifier;
impl FormatRule<JsAnyNamedImportSpecifier> for FormatJsAnyNamedImportSpecifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyNamedImportSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(node) => {
                node.format().fmt(f)
            }
            JsAnyNamedImportSpecifier::JsNamedImportSpecifier(node) => node.format().fmt(f),
            JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(node) => node.format().fmt(f),
        }
    }
}
