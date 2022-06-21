use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsxTagExpression;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxTagExpression;

impl FormatNodeRule<JsxTagExpression> for FormatJsxTagExpression {
    fn fmt_fields(&self, node: &JsxTagExpression, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [node.tag().format()]]
    }
}
