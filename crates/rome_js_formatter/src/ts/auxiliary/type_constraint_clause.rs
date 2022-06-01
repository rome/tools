use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsTypeConstraintClause, TsTypeConstraintClauseFields};

impl FormatNodeFields<TsTypeConstraintClause> for FormatNodeRule<TsTypeConstraintClause> {
    fn format_fields(
        node: &TsTypeConstraintClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsTypeConstraintClauseFields { extends_token, ty } = node.as_fields();

        let extends = extends_token.format();
        let ty = ty.format();
        formatted![formatter, [extends, space_token(), ty]]
    }
}
