use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::{JsAnyFunction, JsArrowFunctionExpression};

impl FormatNode for JsArrowFunctionExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyFunction::from(self.clone()).format(formatter)
    }
}
