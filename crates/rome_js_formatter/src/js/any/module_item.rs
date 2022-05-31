//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyModuleItem;
use crate::prelude::*;
use rome_js_syntax::JsAnyModuleItem;
impl FormatRule<JsAnyModuleItem> for FormatJsAnyModuleItem {
    type Context = JsFormatContext;
    fn format(node: &JsAnyModuleItem, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyModuleItem::JsAnyStatement(node) => node.format().format(f),
            JsAnyModuleItem::JsExport(node) => node.format().format(f),
            JsAnyModuleItem::JsImport(node) => node.format().format(f),
        }
    }
}
