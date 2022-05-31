use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsImplementsClause;
use rome_js_syntax::TsImplementsClauseFields;

impl FormatNodeFields<TsImplementsClause> for FormatNodeRule<TsImplementsClause> {
    fn format_fields(
        node: &TsImplementsClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsImplementsClauseFields {
            implements_token,
            types,
        } = node.as_fields();

        let implements_token = implements_token.format().memoized();
        let types = types.format().memoized();

        Ok(group_elements(formatted![
            formatter,
            [
                if_group_breaks(block_indent(formatted![
                    formatter,
                    [
                        &implements_token,
                        space_token(),
                        soft_block_indent(formatted![formatter, [&types]]?)
                    ]
                ]?)),
                if_group_fits_on_single_line(formatted![
                    formatter,
                    [&implements_token, space_token(), &types]
                ]?),
            ]
        ]?))
    }
}
