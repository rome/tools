use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsSwitchCaseList;
impl ToFormatElement for JsSwitchCaseList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(self.clone()))
    }
}
