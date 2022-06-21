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

        write!(
            f,
            [
                implements_token.format(),
                group_elements(&indent(&format_args![
                    soft_line_break_or_space(),
                    types.format()
                ]))
            ]
        )
    }
}
