//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::JsAnyModuleItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyModuleItem;
impl FormatRule<JsAnyModuleItem> for FormatJsAnyModuleItem {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyModuleItem, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyModuleItem::JsAnyStatement(node) => node.format().fmt(f),
            JsAnyModuleItem::JsExport(node) => node.format().fmt(f),
            JsAnyModuleItem::JsImport(node) => node.format().fmt(f),
        }
    }
}
