use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::{JsAnyClass, JsClassExpression};

impl FormatNodeFields<JsClassExpression> for FormatNodeRule<JsClassExpression> {
    fn format_fields(
        node: &JsClassExpression,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        formatted![formatter, [JsAnyClass::from(node.clone()).format()]]
    }
}
