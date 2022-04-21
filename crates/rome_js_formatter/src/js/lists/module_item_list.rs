use crate::{Format, FormatElement, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsModuleItemList;
impl Format for JsModuleItemList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(self.clone()))
    }
}
