use crate::prelude::*;
use rome_js_syntax::JsxTagExpression;

impl FormatNode for JsxTagExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.tag().format(formatter)
    }
}
