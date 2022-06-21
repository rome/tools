use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::{JsAnyFunction, JsArrowFunctionExpression};

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrowFunctionExpression;

impl FormatNodeRule<JsArrowFunctionExpression> for FormatJsArrowFunctionExpression {
    fn fmt_fields(
        &self,
        node: &JsArrowFunctionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write![f, [JsAnyFunction::from(node.clone()).format()]]
    }
}
