use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsStringType;

impl ToFormatElement for TsStringType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.string_token().format(formatter)
    }
}
