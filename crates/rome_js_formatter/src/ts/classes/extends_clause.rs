use crate::prelude::*;

use rome_formatter::{format_args, write};
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
            [
                extends_token.format(),
                space_token(),
                group_elements(&indent(&format_args![
                    soft_line_break_or_space(),
                    &types.format()
                ]))
            ]
        )
    }
}
