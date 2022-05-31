use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::{JsAnyFunction, JsArrowFunctionExpression};

impl FormatNodeFields<JsArrowFunctionExpression> for FormatNodeRule<JsArrowFunctionExpression> {
    fn format_fields(
        node: &JsArrowFunctionExpression,
        formatter: &Formatter<JsFormatContext>,
    ) -> FormatResult<FormatElement> {
        formatted![formatter, [JsAnyFunction::from(node.clone()).format()]]
    }
}
