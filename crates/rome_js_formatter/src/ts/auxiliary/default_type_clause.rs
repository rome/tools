use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsDefaultTypeClause, TsDefaultTypeClauseFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsDefaultTypeClause;

impl FormatNodeRule<TsDefaultTypeClause> for FormatTsDefaultTypeClause {
    fn fmt_fields(node: &TsDefaultTypeClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsDefaultTypeClauseFields { eq_token, ty } = node.as_fields();
        write![f, [eq_token.format(), space_token(), ty.format()]]
    }
}
