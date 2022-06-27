use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsSuperExpression;
use rome_js_syntax::JsSuperExpressionFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsSuperExpression;

impl FormatNodeRule<JsSuperExpression> for FormatJsSuperExpression {
    fn fmt_fields(&self, node: &JsSuperExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSuperExpressionFields { super_token } = node.as_fields();

        write![f, [super_token.format()]]
    }
}
