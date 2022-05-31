use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsVoidType, TsVoidTypeFields};

impl FormatNodeFields<TsVoidType> for FormatNodeRule<TsVoidType> {
    fn format_fields(node: &TsVoidType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsVoidTypeFields { void_token } = node.as_fields();

        formatted![formatter, [void_token.format()]]
    }
}
