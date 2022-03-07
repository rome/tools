use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsTemplateElementList;
impl ToFormatElement for JsTemplateElementList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(self.clone()))
    }
}
