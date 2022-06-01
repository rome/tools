use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsUndefinedType, TsUndefinedTypeFields};

impl FormatNodeFields<TsUndefinedType> for FormatNodeRule<TsUndefinedType> {
    fn format_fields(
        node: &TsUndefinedType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsUndefinedTypeFields { undefined_token } = node.as_fields();

        formatted![formatter, [undefined_token.format()]]
    }
}
