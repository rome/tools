use crate::prelude::*;
use rome_formatter::write;

use crate::js::declarations::function_declaration::FormatFunction;
use rome_js_syntax::{JsAnyFunction, JsArrowFunctionExpression};

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrowFunctionExpression;

impl FormatNodeRule<JsArrowFunctionExpression> for FormatJsArrowFunctionExpression {
    fn fmt_fields(
        &self,
        node: &JsArrowFunctionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write![f, [FormatFunction::new(&JsAnyFunction::from(node.clone()))]]
    }
}
