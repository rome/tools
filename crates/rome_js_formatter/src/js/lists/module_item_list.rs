use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsModuleItemList;
impl Format for JsModuleItemList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(self.clone()))
    }
}
