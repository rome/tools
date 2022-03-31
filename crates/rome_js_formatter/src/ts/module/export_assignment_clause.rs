use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::format_with_semicolon;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsExportAssignmentClause;
use rome_js_syntax::TsExportAssignmentClauseFields;

impl ToFormatElement for TsExportAssignmentClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsExportAssignmentClauseFields {
            eq_token,
            expression,
            semicolon_token,
        } = self.as_fields();

        format_with_semicolon(
            formatter,
            format_elements![
                eq_token.format(formatter)?,
                space_token(),
                expression.format(formatter)?,
            ],
            semicolon_token,
        )
    }
}
