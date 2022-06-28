use crate::prelude::*;
use crate::utils::JsObjectLike;
use rome_formatter::write;
use rome_js_syntax::JsObjectExpression;

#[derive(Debug, Clone, Default)]
pub struct FormatJsObjectExpression;

impl FormatNodeRule<JsObjectExpression> for FormatJsObjectExpression {
    fn fmt_fields(&self, node: &JsObjectExpression, f: &mut JsFormatter) -> FormatResult<()> {
        write!(f, [JsObjectLike::from(node.clone())])
    }
}
