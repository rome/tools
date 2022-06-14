//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyModuleItem;
use crate::prelude::*;
use rome_js_syntax::JsAnyModuleItem;
impl FormatRule<JsAnyModuleItem> for FormatJsAnyModuleItem {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyModuleItem, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyModuleItem::JsAnyStatement(node) => node.format().fmt(f),
            JsAnyModuleItem::JsExport(node) => node.format().fmt(f),
            JsAnyModuleItem::JsImport(node) => node.format().fmt(f),
        }
    }
}
