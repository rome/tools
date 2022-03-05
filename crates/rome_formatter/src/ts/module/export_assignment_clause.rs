use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_syntax::TsExportAssignmentClause;
use rslint_syntax::TsExportAssignmentClauseFields;

impl ToFormatElement for TsExportAssignmentClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsExportAssignmentClauseFields {
            eq_token,
            expression,
            semicolon_token,
        } = self.as_fields();

        Ok(format_elements![
            eq_token.format(formatter)?,
            space_token(),
            expression.format(formatter)?,
            semicolon_token.format_or(formatter, || token(";"))?,
        ])
    }
}
