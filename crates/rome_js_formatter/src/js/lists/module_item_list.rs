use crate::generated::FormatJsModuleItemList;
use crate::prelude::*;
use rome_js_syntax::JsModuleItemList;

impl FormatRule<JsModuleItemList> for FormatJsModuleItemList {
    fn format(node: &JsModuleItemList, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(node))
    }
}
