use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};
use crate::FormatNodeFields;
use rome_js_syntax::{TsStringLiteralType, TsStringLiteralTypeFields};

impl FormatNodeFields<TsStringLiteralType> for FormatNodeRule<TsStringLiteralType> {
    fn format_fields(
        node: &TsStringLiteralType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsStringLiteralTypeFields { literal_token } = node.as_fields();

        formatted![
            formatter,
            [FormatLiteralStringToken::new(
                &literal_token?,
                StringLiteralParentKind::Expression
            )]
        ]
    }
}
