//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::JsAnyExportNamedSpecifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyExportNamedSpecifier;
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
