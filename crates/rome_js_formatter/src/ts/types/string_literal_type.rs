use crate::prelude::*;
use crate::utils::format_string_literal_token;
use crate::FormatNodeFields;
use rome_js_syntax::{TsStringLiteralType, TsStringLiteralTypeFields};

impl FormatNodeFields<TsStringLiteralType> for FormatNodeRule<TsStringLiteralType> {
    fn format_fields(
        node: &TsStringLiteralType,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let TsStringLiteralTypeFields { literal_token } = node.as_fields();

        Ok(format_string_literal_token(literal_token?, formatter))
    }
}
