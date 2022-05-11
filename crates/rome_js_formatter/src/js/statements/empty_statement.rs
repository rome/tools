use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsEmptyStatement;
use rome_js_syntax::JsEmptyStatementFields;

impl FormatNodeFields<JsEmptyStatement> for FormatNodeRule<JsEmptyStatement> {
    fn format_fields(
        node: &JsEmptyStatement,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsEmptyStatementFields { semicolon_token } = node.as_fields();

        Ok(formatter.format_replaced(&semicolon_token?, empty_element()))
    }
}
