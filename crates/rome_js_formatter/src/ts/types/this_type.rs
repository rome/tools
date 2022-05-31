use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsThisType, TsThisTypeFields};

impl FormatNodeFields<TsThisType> for FormatNodeRule<TsThisType> {
    fn format_fields(node: &TsThisType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsThisTypeFields { this_token } = node.as_fields();

        formatted![formatter, [this_token.format()]]
    }
}
