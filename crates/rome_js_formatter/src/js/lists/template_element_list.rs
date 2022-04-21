use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsTemplateElementList;
impl Format for JsTemplateElementList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(self.clone()))
    }
}
