use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::{JsAnyClass, JsClassExpression};

impl FormatNode for JsClassExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyClass::from(self.clone()).format(formatter)
    }
}
