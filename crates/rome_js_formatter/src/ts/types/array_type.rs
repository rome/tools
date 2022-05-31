use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsArrayType, TsArrayTypeFields};

impl FormatNodeFields<TsArrayType> for FormatNodeRule<TsArrayType> {
    fn format_fields(node: &TsArrayType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsArrayTypeFields {
            l_brack_token,
            element_type,
            r_brack_token,
        } = node.as_fields();
        formatted![
            formatter,
            [
                element_type.format(),
                l_brack_token.format(),
                r_brack_token.format(),
            ]
        ]
    }
}
