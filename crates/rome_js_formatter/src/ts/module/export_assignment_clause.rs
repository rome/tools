use crate::prelude::*;
use crate::utils::format_with_semicolon;
use rome_js_syntax::TsExportAssignmentClause;
use rome_js_syntax::TsExportAssignmentClauseFields;

impl FormatNode for TsExportAssignmentClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsExportAssignmentClauseFields {
            eq_token,
            expression,
            semicolon_token,
        } = self.as_fields();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                eq_token.format(formatter)?,
                space_token(),
                expression.format(formatter)?,
            ]?,
            semicolon_token,
        )
    }
}
