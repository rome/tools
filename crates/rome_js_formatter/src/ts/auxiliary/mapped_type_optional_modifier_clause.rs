use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsMappedTypeOptionalModifierClause;
use rome_js_syntax::TsMappedTypeOptionalModifierClauseFields;

impl FormatNodeFields<TsMappedTypeOptionalModifierClause>
    for FormatNodeRule<TsMappedTypeOptionalModifierClause>
{
    fn format_fields(
        node: &TsMappedTypeOptionalModifierClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsMappedTypeOptionalModifierClauseFields {
            operator_token,
            question_mark_token,
        } = node.as_fields();

        formatted![
            formatter,
            [operator_token.format(), question_mark_token.format()]
        ]
    }
}
