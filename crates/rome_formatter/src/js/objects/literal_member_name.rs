use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsLiteralMemberName;
use rslint_parser::ast::JsLiteralMemberNameFields;

impl ToFormatElement for JsLiteralMemberName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsLiteralMemberNameFields { value } = self.as_fields();

        value.format(formatter)
    }
}
