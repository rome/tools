use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsLiteralExportName;
use rslint_parser::ast::JsLiteralExportNameFields;

impl ToFormatElement for JsLiteralExportName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsLiteralExportNameFields { value } = self.as_fields();

        value.format(formatter)
    }
}
