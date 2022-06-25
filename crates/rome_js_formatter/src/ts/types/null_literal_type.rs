use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsNullLiteralType, TsNullLiteralTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsNullLiteralType;

impl FormatNodeRule<TsNullLiteralType> for FormatTsNullLiteralType {
    fn fmt_fields(&self, node: &TsNullLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNullLiteralTypeFields { literal_token } = node.as_fields();
        write![f, [literal_token.format()]]
    }
}
