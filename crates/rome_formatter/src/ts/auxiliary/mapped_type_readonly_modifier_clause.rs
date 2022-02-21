use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsMappedTypeReadonlyModifierClause;
use rslint_parser::ast::TsMappedTypeReadonlyModifierClauseFields;

impl ToFormatElement for TsMappedTypeReadonlyModifierClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
