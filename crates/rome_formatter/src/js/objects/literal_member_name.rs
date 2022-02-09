use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsLiteralMemberName;

impl ToFormatElement for JsLiteralMemberName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.value().format(formatter)
    }
}
