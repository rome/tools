use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsTypeConstraintClause, TsTypeConstraintClauseFields};

impl FormatNodeFields<TsTypeConstraintClause> for FormatNodeRule<TsTypeConstraintClause> {
    fn fmt_fields(node: &TsTypeConstraintClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeConstraintClauseFields { extends_token, ty } = node.as_fields();

        let extends = extends_token.format();
        let ty = ty.format();
        write![f, [extends, space_token(), ty]]
    }
}
