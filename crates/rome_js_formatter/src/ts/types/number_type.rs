use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsNumberType, TsNumberTypeFields};

impl FormatNodeFields<TsNumberType> for FormatNodeRule<TsNumberType> {
    fn format_fields(node: &TsNumberType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsNumberTypeFields { number_token } = node.as_fields();

        formatted![formatter, [number_token.format()]]
    }
}
