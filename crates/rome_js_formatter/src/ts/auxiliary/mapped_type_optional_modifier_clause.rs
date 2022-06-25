use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsMappedTypeOptionalModifierClause;
use rome_js_syntax::TsMappedTypeOptionalModifierClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsMappedTypeOptionalModifierClause;

impl FormatNodeRule<TsMappedTypeOptionalModifierClause>
    for FormatTsMappedTypeOptionalModifierClause
{
    fn fmt_fields(
        &self,
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
