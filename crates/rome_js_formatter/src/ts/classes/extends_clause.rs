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

        let extends_token = extends_token.format().memoized();
        let types = types.format().memoized();

        write!(
            f,
            [group_elements(&format_args!(
                if_group_breaks(&block_indent(&format_args![
                    &extends_token,
                    space_token(),
                    soft_block_indent(&types)
                ])),
                if_group_fits_on_line(&format_args![&extends_token, space_token(), &types]),
            ))]
        )
    }
}
