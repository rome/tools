use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsExtendsClause, TsExtendsClauseFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsExtendsClause;

impl FormatNodeRule<TsExtendsClause> for FormatTsExtendsClause {
    fn fmt_fields(&self, node: &TsExtendsClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsExtendsClauseFields {
            extends_token,
            types,
        } = node.as_fields();

        write!(
            f,
            [extends_token.format(), space(), &indent(&types.format())]
        )
    }
}
