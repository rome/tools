use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsMappedTypeReadonlyModifierClause;
use rome_js_syntax::TsMappedTypeReadonlyModifierClauseFields;

impl FormatNode for TsMappedTypeReadonlyModifierClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsMappedTypeReadonlyModifierClauseFields {
            operator_token,
            readonly_token,
        } = self.as_fields();
        formatted![
            formatter,
            operator_token.format(formatter)?,
            readonly_token.format(formatter)?
        ]
    }
}
