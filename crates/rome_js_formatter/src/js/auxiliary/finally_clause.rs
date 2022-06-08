use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsFinallyClause;
use rome_js_syntax::JsFinallyClauseFields;

impl FormatNodeFields<JsFinallyClause> for FormatNodeRule<JsFinallyClause> {
    fn fmt_fields(node: &JsFinallyClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsFinallyClauseFields {
            finally_token,
            body,
        } = node.as_fields();

        write![f, [finally_token.format(), space_token(), body.format()]]
    }
}
