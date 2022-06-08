use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsMappedTypeOptionalModifierClause;
use rome_js_syntax::TsMappedTypeOptionalModifierClauseFields;

impl FormatNodeFields<TsMappedTypeOptionalModifierClause>
    for FormatNodeRule<TsMappedTypeOptionalModifierClause>
{
    fn fmt_fields(
        node: &TsMappedTypeOptionalModifierClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsMappedTypeOptionalModifierClauseFields {
            operator_token,
            question_mark_token,
        } = node.as_fields();

        write![f, [operator_token.format(), question_mark_token.format()]]
    }
}
