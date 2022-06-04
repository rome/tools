use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsBigIntLiteralType, TsBigIntLiteralTypeFields};

impl FormatNodeFields<TsBigIntLiteralType> for FormatNodeRule<TsBigIntLiteralType> {
    fn fmt_fields(node: &TsBigIntLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsBigIntLiteralTypeFields {
            minus_token,
            literal_token,
        } = node.as_fields();

        write![f, [minus_token.format(), literal_token.format()]]
    }
}
