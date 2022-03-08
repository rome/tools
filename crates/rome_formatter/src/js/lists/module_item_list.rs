use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsModuleItemList;
impl ToFormatElement for JsModuleItemList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(self.clone()))
    }
}
