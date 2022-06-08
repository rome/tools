use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsCatchClause;
use rome_js_syntax::JsCatchClauseFields;

impl FormatNodeFields<JsCatchClause> for FormatNodeRule<JsCatchClause> {
    fn fmt_fields(node: &JsCatchClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsCatchClauseFields {
            catch_token,
            declaration,
            body,
        } = node.as_fields();

        write!(f, [catch_token.format(), space_token()])?;

        if let Some(declaration) = declaration {
            write![f, [declaration.format(), space_token()]]?;
        }

        write!(f, [body.format()])
    }
}
