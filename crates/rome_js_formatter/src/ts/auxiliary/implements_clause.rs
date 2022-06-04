use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::TsImplementsClause;
use rome_js_syntax::TsImplementsClauseFields;

impl FormatNodeFields<TsImplementsClause> for FormatNodeRule<TsImplementsClause> {
    fn fmt_fields(node: &TsImplementsClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsImplementsClauseFields {
            implements_token,
            types,
        } = node.as_fields();

        let implements_token = implements_token.format().memoized();
        let types = types.format().memoized();

        write!(
            f,
            [group_elements(&format_args![
                if_group_breaks(&block_indent(&format_args![
                    &implements_token,
                    space_token(),
                    soft_block_indent(&types)
                ])),
                if_group_fits_on_line(&format_args![&implements_token, space_token(), &types]),
            ])]
        )
    }
}
