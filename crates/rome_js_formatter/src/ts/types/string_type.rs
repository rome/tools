use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsStringType, TsStringTypeFields};

impl FormatNodeFields<TsStringType> for FormatNodeRule<TsStringType> {
    fn format_fields(node: &TsStringType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsStringTypeFields { string_token } = node.as_fields();

        formatted![formatter, [string_token.format()]]
    }
}
