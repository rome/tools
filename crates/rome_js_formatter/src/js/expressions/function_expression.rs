use crate::prelude::*;

use crate::js::declarations::function_declaration::FormatFunction;
use rome_formatter::write;
use rome_js_syntax::{JsAnyFunction, JsFunctionExpression};

#[derive(Debug, Clone, Default)]
pub struct FormatJsFunctionExpression;

impl FormatNodeRule<JsFunctionExpression> for FormatJsFunctionExpression {
    fn fmt_fields(&self, node: &JsFunctionExpression, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [FormatFunction::new(&JsAnyFunction::from(node.clone()))]]
    }
}
