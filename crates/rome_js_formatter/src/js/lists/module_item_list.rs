use crate::formatter::FormatNodeExtension;
use crate::generated::FormatJsModuleItemList;
use crate::prelude::*;
use rome_js_syntax::JsModuleItemList;

impl FormatRule<JsModuleItemList> for FormatJsModuleItemList {
    type Context = JsFormatContext;

    fn format(node: &JsModuleItemList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for module_item in node {
            join.entry(module_item.syntax(), &module_item.format_or_verbatim());
        }

        join.finish()
    }
}
