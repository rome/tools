use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsNonPrimitiveType, TsNonPrimitiveTypeFields};

impl FormatNodeFields<TsNonPrimitiveType> for FormatNodeRule<TsNonPrimitiveType> {
    fn format_fields(
        node: &TsNonPrimitiveType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsNonPrimitiveTypeFields { object_token } = node.as_fields();

        formatted![formatter, [object_token.format()]]
    }
}
