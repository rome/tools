use crate::{empty_element, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsEmptyStatement;
use rome_js_syntax::JsEmptyStatementFields;

impl ToFormatElement for JsEmptyStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsEmptyStatementFields { semicolon_token } = self.as_fields();

        Ok(formatter.format_replaced(&semicolon_token?, empty_element()))
    }
}
