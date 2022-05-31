use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsFinallyClause;
use rome_js_syntax::JsFinallyClauseFields;

impl FormatNodeFields<JsFinallyClause> for FormatNodeRule<JsFinallyClause> {
    fn format_fields(
        node: &JsFinallyClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsFinallyClauseFields {
            finally_token,
            body,
        } = node.as_fields();

        formatted![
            formatter,
            [finally_token.format(), space_token(), body.format()]
        ]
    }
}
