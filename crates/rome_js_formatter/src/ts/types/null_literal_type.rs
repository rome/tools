use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsNullLiteralType, TsNullLiteralTypeFields};

impl FormatNodeFields<TsNullLiteralType> for FormatNodeRule<TsNullLiteralType> {
    fn fmt_fields(node: &TsNullLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNullLiteralTypeFields { literal_token } = node.as_fields();
        write![f, [literal_token.format()]]
    }
}
