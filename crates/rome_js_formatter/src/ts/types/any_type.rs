use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsAnyType, TsAnyTypeFields};

impl FormatNodeFields<TsAnyType> for FormatNodeRule<TsAnyType> {
    fn format_fields(node: &TsAnyType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsAnyTypeFields { any_token } = node.as_fields();

        formatted![formatter, [any_token.format()]]
    }
}
