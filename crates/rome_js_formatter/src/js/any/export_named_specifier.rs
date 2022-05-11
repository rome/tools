//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyExportNamedSpecifier;
use crate::prelude::*;
use rome_js_syntax::JsAnyExportNamedSpecifier;
impl FormatRule<JsAnyExportNamedSpecifier> for FormatJsAnyExportNamedSpecifier {
    fn format(
        node: &JsAnyExportNamedSpecifier,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyExportNamedSpecifier::JsExportNamedShorthandSpecifier(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyExportNamedSpecifier::JsExportNamedSpecifier(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
