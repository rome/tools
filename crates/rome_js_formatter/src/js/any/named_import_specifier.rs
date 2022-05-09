//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyNamedImportSpecifier;
use crate::prelude::*;
use rome_js_syntax::JsAnyNamedImportSpecifier;
impl FormatRule<JsAnyNamedImportSpecifier> for FormatJsAnyNamedImportSpecifier {
    fn format(
        node: &JsAnyNamedImportSpecifier,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyNamedImportSpecifier::JsNamedImportSpecifier(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(node) => {
                formatted![formatter, node.format()]
            }
        }
    }
}
