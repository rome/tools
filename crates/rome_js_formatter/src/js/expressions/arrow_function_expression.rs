use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::{JsAnyFunction, JsArrowFunctionExpression};

impl FormatNodeFields<JsArrowFunctionExpression> for FormatNodeRule<JsArrowFunctionExpression> {
    fn fmt_fields(node: &JsArrowFunctionExpression, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyFunction::from(node.clone()).format()]]
    }
}
