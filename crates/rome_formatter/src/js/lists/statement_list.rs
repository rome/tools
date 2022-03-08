use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsStatementList;
impl ToFormatElement for JsStatementList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(self.clone()))
    }
}
