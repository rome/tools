use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsElseClause;
use rome_js_syntax::JsElseClauseFields;

impl FormatNodeFields<JsElseClause> for FormatNodeRule<JsElseClause> {
    fn fmt_fields(node: &JsElseClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsElseClauseFields {
            else_token,
            alternate,
        } = node.as_fields();

        write![f, [else_token.format(), space_token(), alternate.format(),]]
    }
}
