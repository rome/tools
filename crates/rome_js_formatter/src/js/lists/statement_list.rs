use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsStatementList;
impl Format for JsStatementList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(self.clone()))
    }
}
