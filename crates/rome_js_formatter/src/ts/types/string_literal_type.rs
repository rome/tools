use crate::prelude::*;
use crate::utils::FormatLiteralStringToken;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsStringLiteralType, TsStringLiteralTypeFields};

impl FormatNodeFields<TsStringLiteralType> for FormatNodeRule<TsStringLiteralType> {
    fn format_fields(node: &TsStringLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsStringLiteralTypeFields { literal_token } = node.as_fields();

        write!(f, [FormatLiteralStringToken::from_string(&literal_token?)])
    }
}
