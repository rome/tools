use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::TsAnyType;

impl ToFormatElement for TsAnyType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.any_token().format(formatter)
    }
}
