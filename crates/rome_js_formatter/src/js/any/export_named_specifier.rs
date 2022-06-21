//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyExportNamedSpecifier;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyExportNamedSpecifier;
impl FormatRule<JsAnyExportNamedSpecifier> for FormatJsAnyExportNamedSpecifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyExportNamedSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyExportNamedSpecifier::JsExportNamedShorthandSpecifier(node) => {
                node.format().fmt(f)
            }
            JsAnyExportNamedSpecifier::JsExportNamedSpecifier(node) => node.format().fmt(f),
        }
    }
}
