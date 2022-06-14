//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyNamedImportSpecifier;
use crate::prelude::*;
use rome_js_syntax::JsAnyNamedImportSpecifier;
impl FormatRule<JsAnyNamedImportSpecifier> for FormatJsAnyNamedImportSpecifier {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyNamedImportSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(node) => {
                node.format().fmt(f)
            }
            JsAnyNamedImportSpecifier::JsNamedImportSpecifier(node) => node.format().fmt(f),
            JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(node) => node.format().fmt(f),
        }
    }
}
