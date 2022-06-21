use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsMappedTypeReadonlyModifierClause;
use rome_js_syntax::TsMappedTypeReadonlyModifierClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsMappedTypeReadonlyModifierClause;

impl FormatNodeRule<TsMappedTypeReadonlyModifierClause>
    for FormatTsMappedTypeReadonlyModifierClause
{
    fn fmt_fields(
        node: &TsMappedTypeReadonlyModifierClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsMappedTypeReadonlyModifierClauseFields {
            operator_token,
            readonly_token,
        } = node.as_fields();
        write![f, [operator_token.format(), readonly_token.format()]]
    }
}
