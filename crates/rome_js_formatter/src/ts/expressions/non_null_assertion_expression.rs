use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsNonNullAssertionExpression;
use rome_js_syntax::TsNonNullAssertionExpressionFields;

impl FormatNodeFields<TsNonNullAssertionExpression>
    for FormatNodeRule<TsNonNullAssertionExpression>
{
    fn fmt_fields(node: &TsNonNullAssertionExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNonNullAssertionExpressionFields {
            expression,
            excl_token,
        } = node.as_fields();

        write![f, [expression.format(), excl_token.format()]]
    }
}
