use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsNonPrimitiveType;

impl ToFormatElement for TsNonPrimitiveType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.object_token().format(formatter)
    }
}
