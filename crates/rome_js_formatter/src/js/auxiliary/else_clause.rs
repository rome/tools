use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsElseClause;
use rome_js_syntax::JsElseClauseFields;

impl FormatNodeFields<JsElseClause> for FormatNodeRule<JsElseClause> {
    fn format_fields(node: &JsElseClause, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let JsElseClauseFields {
            else_token,
            alternate,
        } = node.as_fields();

        formatted![
            formatter,
            [else_token.format(), space_token(), alternate.format(),]
        ]
    }
}
