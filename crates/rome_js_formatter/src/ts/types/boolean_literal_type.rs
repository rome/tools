use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsBooleanLiteralType, TsBooleanLiteralTypeFields};

impl FormatNodeFields<TsBooleanLiteralType> for FormatNodeRule<TsBooleanLiteralType> {
    fn fmt_fields(node: &TsBooleanLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsBooleanLiteralTypeFields { literal } = node.as_fields();
        write![f, [literal.format()]]
    }
}
