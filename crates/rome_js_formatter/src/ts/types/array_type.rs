use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsArrayType, TsArrayTypeFields};

impl FormatNodeFields<TsArrayType> for FormatNodeRule<TsArrayType> {
    fn fmt_fields(node: &TsArrayType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsArrayTypeFields {
            l_brack_token,
            element_type,
            r_brack_token,
        } = node.as_fields();
        write![
            f,
            [
                element_type.format(),
                l_brack_token.format(),
                r_brack_token.format(),
            ]
        ]
    }
}
