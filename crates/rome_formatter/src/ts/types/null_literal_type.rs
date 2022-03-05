use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::TsNullLiteralType;

impl ToFormatElement for TsNullLiteralType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.literal_token().format(formatter)
    }
}
