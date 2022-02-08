use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsBooleanType;

impl ToFormatElement for TsBooleanType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.boolean_token().format(formatter)
    }
}
