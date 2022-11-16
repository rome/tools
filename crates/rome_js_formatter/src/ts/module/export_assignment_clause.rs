use crate::prelude::*;
use crate::utils::FormatStatementSemicolon;

use rome_formatter::write;
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
            [
                eq_token.format(),
                space(),
                expression.format(),
                FormatStatementSemicolon::new(semicolon_token.as_ref())
            ]
        )
    }
}
