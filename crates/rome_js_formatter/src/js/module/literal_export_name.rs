use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};
use crate::FormatNodeFields;
use rome_js_syntax::JsLiteralExportName;
use rome_js_syntax::JsLiteralExportNameFields;

impl FormatNodeFields<JsLiteralExportName> for FormatNodeRule<JsLiteralExportName> {
    fn format_fields(
        node: &JsLiteralExportName,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsLiteralExportNameFields { value } = node.as_fields();

        formatted![
            formatter,
            [FormatLiteralStringToken::new(
                &value?,
                StringLiteralParentKind::Expression
            )]
        ]
    }
}
