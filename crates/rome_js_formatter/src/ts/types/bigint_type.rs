use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsBigintType, TsBigintTypeFields};

impl FormatNodeFields<TsBigintType> for FormatNodeRule<TsBigintType> {
    fn format_fields(node: &TsBigintType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsBigintTypeFields { bigint_token } = node.as_fields();

        formatted![formatter, [bigint_token.format()]]
    }
}
