use crate::generated::FormatJsModuleItemList;
use crate::prelude::*;
use rome_js_syntax::JsModuleItemList;

impl FormatRule<JsModuleItemList> for FormatJsModuleItemList {
    type Context = JsFormatContext;

    fn format(
        node: &JsModuleItemList,
        formatter: &Formatter<JsFormatContext>,
    ) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(node))
    }
}
