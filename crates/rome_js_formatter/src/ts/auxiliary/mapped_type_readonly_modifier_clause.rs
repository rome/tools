use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsMappedTypeReadonlyModifierClause;
use rome_js_syntax::TsMappedTypeReadonlyModifierClauseFields;

impl FormatNode for TsMappedTypeReadonlyModifierClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsMappedTypeReadonlyModifierClauseFields {
            operator_token,
            readonly_token,
        } = self.as_fields();
        Ok(format_elements![
            operator_token.format(formatter)?,
            readonly_token.format(formatter)?
        ])
    }
}
