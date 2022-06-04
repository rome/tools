use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsNumberLiteralType, TsNumberLiteralTypeFields};

impl FormatNodeFields<TsNumberLiteralType> for FormatNodeRule<TsNumberLiteralType> {
    fn fmt_fields(node: &TsNumberLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNumberLiteralTypeFields {
            minus_token,
            literal_token,
        } = node.as_fields();
        write![f, [minus_token.format(), literal_token.format()]]
    }
}
