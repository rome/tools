use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsMappedTypeOptionalModifierClause;
use rome_js_syntax::TsMappedTypeOptionalModifierClauseFields;

impl ToFormatElement for TsMappedTypeOptionalModifierClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsMappedTypeOptionalModifierClauseFields {
            operator_token,
            question_mark_token,
        } = self.as_fields();

        Ok(format_elements![
            operator_token.format(formatter)?,
            question_mark_token.format(formatter)?
        ])
    }
}
