use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{JsxAttributeInitializerClause, JsxAttributeInitializerClauseFields};

impl FormatNodeFields<JsxAttributeInitializerClause>
    for FormatNodeRule<JsxAttributeInitializerClause>
{
    fn fmt_fields(node: &JsxAttributeInitializerClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxAttributeInitializerClauseFields { eq_token, value } = node.as_fields();

        write![f, [eq_token.format(), value.format()]]
    }
}
