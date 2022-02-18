use crate::{empty_element, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsEmptyStatement;
use rslint_parser::ast::JsEmptyStatementFields;

impl ToFormatElement for JsEmptyStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsEmptyStatementFields { semicolon_token } = self.as_fields();

        formatter.format_replaced(&semicolon_token?, empty_element())
    }
}
