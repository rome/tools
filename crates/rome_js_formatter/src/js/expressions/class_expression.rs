use crate::prelude::*;
use crate::utils::format_class::FormatClass;

use rome_js_syntax::JsClassExpression;

#[derive(Debug, Clone, Default)]
pub struct FormatJsClassExpression;

impl FormatNodeRule<JsClassExpression> for FormatJsClassExpression {
    fn fmt_fields(&self, node: &JsClassExpression, f: &mut JsFormatter) -> FormatResult<()> {
        FormatClass::from(&node.clone().into()).fmt(f)
    }
}
