use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsBooleanType, TsBooleanTypeFields};

impl FormatNodeFields<TsBooleanType> for FormatNodeRule<TsBooleanType> {
    fn format_fields(node: &TsBooleanType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsBooleanTypeFields { boolean_token } = node.as_fields();

        formatted![formatter, [boolean_token.format()]]
    }
}
