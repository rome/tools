use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsStringLiteralType, TsStringLiteralTypeFields};

impl FormatNodeFields<TsStringLiteralType> for FormatNodeRule<TsStringLiteralType> {
    fn fmt_fields(node: &TsStringLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsStringLiteralTypeFields { literal_token } = node.as_fields();

        write!(
            f,
            [FormatLiteralStringToken::new(
                &literal_token?,
                StringLiteralParentKind::Expression
            )]
        )
    }
}
