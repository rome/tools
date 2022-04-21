use crate::{empty_element, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsEmptyStatement;
use rome_js_syntax::JsEmptyStatementFields;

impl FormatNode for JsEmptyStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsEmptyStatementFields { semicolon_token } = self.as_fields();

        Ok(formatter.format_replaced(&semicolon_token?, empty_element()))
    }
}
