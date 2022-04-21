use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsxTagExpression;

impl FormatNode for JsxTagExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.tag().format(formatter)
    }
}
