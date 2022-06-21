use crate::prelude::*;
use rome_js_syntax::JsModuleItemList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsModuleItemList;

impl FormatRule<JsModuleItemList> for FormatJsModuleItemList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsModuleItemList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for module_item in node {
            join.entry(module_item.syntax(), &format_or_verbatim(&module_item));
        }

        join.finish()
    }
}
