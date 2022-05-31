use crate::prelude::*;
use crate::utils::format_with_semicolon;

use crate::FormatNodeFields;
use rome_js_syntax::JsExportDefaultExpressionClause;
use rome_js_syntax::JsExportDefaultExpressionClauseFields;

impl FormatNodeFields<JsExportDefaultExpressionClause>
    for FormatNodeRule<JsExportDefaultExpressionClause>
{
    fn format_fields(
        node: &JsExportDefaultExpressionClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsExportDefaultExpressionClauseFields {
            default_token,
            expression,
            semicolon_token,
        } = node.as_fields();

        let default_token = default_token.format();
        let class = expression.format();

        format_with_semicolon(
            formatter,
            formatted![formatter, [default_token, space_token(), class]]?,
            semicolon_token,
        )
    }
}
