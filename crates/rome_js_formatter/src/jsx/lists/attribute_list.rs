use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsxAttributeList;
impl Format for JsxAttributeList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(self.clone()))
    }
}
