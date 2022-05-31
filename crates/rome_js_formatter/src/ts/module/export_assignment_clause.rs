use crate::prelude::*;
use crate::utils::format_with_semicolon;
use crate::FormatNodeFields;
use rome_js_syntax::TsExportAssignmentClause;
use rome_js_syntax::TsExportAssignmentClauseFields;

impl FormatNodeFields<TsExportAssignmentClause> for FormatNodeRule<TsExportAssignmentClause> {
    fn format_fields(
        node: &TsExportAssignmentClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsExportAssignmentClauseFields {
            eq_token,
            expression,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                [eq_token.format(), space_token(), expression.format(),]
            ]?,
            semicolon_token,
        )
    }
}
