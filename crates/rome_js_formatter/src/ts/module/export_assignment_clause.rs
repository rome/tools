use crate::prelude::*;
use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::TsExportAssignmentClause;
use rome_js_syntax::TsExportAssignmentClauseFields;

impl FormatNodeFields<TsExportAssignmentClause> for FormatNodeRule<TsExportAssignmentClause> {
    fn fmt_fields(node: &TsExportAssignmentClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsExportAssignmentClauseFields {
            eq_token,
            expression,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(eq_token.format(), space_token(), expression.format()),
                semicolon_token.as_ref()
            )]
        )
    }
}
