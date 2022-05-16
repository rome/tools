use crate::generated::FormatJsModuleItemList;
use crate::prelude::*;
use rome_js_syntax::JsModuleItemList;

impl FormatRule<JsModuleItemList> for FormatJsModuleItemList {
    type Options = JsFormatOptions;

    fn format(
        node: &JsModuleItemList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(node))
    }
}
