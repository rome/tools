use crate::prelude::*;
use crate::utils::FormatWithSemicolon;

use rome_formatter::{format_args, write};
use rome_js_syntax::TsExportAssignmentClause;
use rome_js_syntax::TsExportAssignmentClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsExportAssignmentClause;

impl FormatNodeRule<TsExportAssignmentClause> for FormatTsExportAssignmentClause {
    fn fmt_fields(&self, node: &TsExportAssignmentClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsExportAssignmentClauseFields {
            eq_token,
            expression,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(eq_token.format(), space(), expression.format()),
                semicolon_token.as_ref()
            )]
        )
    }
}
