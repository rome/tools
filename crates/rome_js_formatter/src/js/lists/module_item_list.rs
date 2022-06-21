use crate::prelude::*;
use rome_js_syntax::JsModuleItemList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsModuleItemList;

impl FormatNodeRule<JsModuleItemList> for FormatJsModuleItemList {
    fn fmt_fields(&self, node: &JsModuleItemList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for module_item in node {
            join.entry(module_item.syntax(), &format_or_verbatim(&module_item));
        }

        join.finish()
    }
}
