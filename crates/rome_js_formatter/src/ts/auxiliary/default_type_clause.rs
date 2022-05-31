use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsDefaultTypeClause, TsDefaultTypeClauseFields};

impl FormatNodeFields<TsDefaultTypeClause> for FormatNodeRule<TsDefaultTypeClause> {
    fn format_fields(
        node: &TsDefaultTypeClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsDefaultTypeClauseFields { eq_token, ty } = node.as_fields();
        formatted![formatter, [eq_token.format(), space_token(), ty.format()]]
    }
}
