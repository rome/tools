use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsNumberLiteralType, TsNumberLiteralTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsNumberLiteralType;

impl FormatNodeRule<TsNumberLiteralType> for FormatTsNumberLiteralType {
    fn fmt_fields(node: &TsNumberLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNumberLiteralTypeFields {
            minus_token,
            literal_token,
        } = node.as_fields();
        write![f, [minus_token.format(), literal_token.format()]]
    }
}
