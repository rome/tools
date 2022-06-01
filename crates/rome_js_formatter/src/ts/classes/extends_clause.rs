use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsExtendsClause, TsExtendsClauseFields};

impl FormatNodeFields<TsExtendsClause> for FormatNodeRule<TsExtendsClause> {
    fn format_fields(
        node: &TsExtendsClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsExtendsClauseFields {
            extends_token,
            types,
        } = node.as_fields();

        let extends_token = extends_token.format().memoized();
        let types = types.format().memoized();

        Ok(group_elements(formatted![
            formatter,
            [
                if_group_breaks(block_indent(formatted![
                    formatter,
                    [
                        &extends_token,
                        space_token(),
                        soft_block_indent(formatted![formatter, [&types]]?)
                    ]
                ]?)),
                if_group_fits_on_single_line(formatted![
                    formatter,
                    [&extends_token, space_token(), &types]
                ]?),
            ]
        ]?))
    }
}
