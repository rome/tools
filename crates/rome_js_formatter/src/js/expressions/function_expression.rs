use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::{JsAnyFunction, JsFunctionExpression};

impl FormatNodeFields<JsFunctionExpression> for FormatNodeRule<JsFunctionExpression> {
    fn format_fields(
        node: &JsFunctionExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        formatted![formatter, [JsAnyFunction::from(node.clone()).format()]]
    }
}
