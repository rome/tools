use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsMappedTypeAsClause, TsMappedTypeAsClauseFields};

impl FormatNodeFields<TsMappedTypeAsClause> for FormatNodeRule<TsMappedTypeAsClause> {
    fn fmt_fields(node: &TsMappedTypeAsClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsMappedTypeAsClauseFields { as_token, ty } = node.as_fields();

        write![f, [as_token.format(), space_token(), ty.format()]]
    }
}
