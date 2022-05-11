//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyModuleItem;
use crate::prelude::*;
use rome_js_syntax::JsAnyModuleItem;
impl FormatRule<JsAnyModuleItem> for FormatJsAnyModuleItem {
    fn format(node: &JsAnyModuleItem, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyModuleItem::JsAnyStatement(node) => formatted![formatter, [node.format()]],
            JsAnyModuleItem::JsExport(node) => formatted![formatter, [node.format()]],
            JsAnyModuleItem::JsImport(node) => formatted![formatter, [node.format()]],
        }
    }
}
