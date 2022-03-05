use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::TsVoidType;

impl ToFormatElement for TsVoidType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.void_token().format(formatter)
    }
}
