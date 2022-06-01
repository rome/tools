use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsMappedTypeAsClause, TsMappedTypeAsClauseFields};

impl FormatNodeFields<TsMappedTypeAsClause> for FormatNodeRule<TsMappedTypeAsClause> {
    fn format_fields(
        node: &TsMappedTypeAsClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsMappedTypeAsClauseFields { as_token, ty } = node.as_fields();

        formatted![
            formatter,
            [
                as_token
                    .format()
                    .with(|as_token| { formatted![formatter, [as_token, space_token()]] }),
                ty.format()
            ]
        ]
    }
}
