use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsNumberType;

impl ToFormatElement for TsNumberType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.number_token().format(formatter)
    }
}
