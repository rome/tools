use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{JsxAttributeInitializerClause, JsxAttributeInitializerClauseFields};

impl FormatNodeFields<JsxAttributeInitializerClause>
    for FormatNodeRule<JsxAttributeInitializerClause>
{
    fn format_fields(
        node: &JsxAttributeInitializerClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsxAttributeInitializerClauseFields { eq_token, value } = node.as_fields();

        formatted![formatter, [eq_token.format(), value.format()]]
    }
}
