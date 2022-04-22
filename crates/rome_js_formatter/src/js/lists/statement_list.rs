use crate::{Format, FormatElement, Formatter, JsFormatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsStatementList;
impl Format for JsStatementList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(self.clone()))
    }
}
