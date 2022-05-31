use crate::formatter::TryFormatNodeListExtension;
use crate::generated::FormatJsModuleItemList;
use crate::prelude::*;
use rome_js_syntax::JsModuleItemList;

impl FormatRule<JsModuleItemList> for FormatJsModuleItemList {
    type Context = JsFormatContext;

    fn format(node: &JsModuleItemList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&hard_line_break())
            .entries(node.try_format_nodes())
            .finish()
    }
}
