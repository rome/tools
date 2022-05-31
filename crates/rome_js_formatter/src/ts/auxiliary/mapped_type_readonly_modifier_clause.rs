use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsMappedTypeReadonlyModifierClause;
use rome_js_syntax::TsMappedTypeReadonlyModifierClauseFields;

impl FormatNodeFields<TsMappedTypeReadonlyModifierClause>
    for FormatNodeRule<TsMappedTypeReadonlyModifierClause>
{
    fn format_fields(
        node: &TsMappedTypeReadonlyModifierClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsMappedTypeReadonlyModifierClauseFields {
            operator_token,
            readonly_token,
        } = node.as_fields();
        formatted![
            formatter,
            [operator_token.format(), readonly_token.format()]
        ]
    }
}
