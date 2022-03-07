use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsBooleanLiteralType;

impl ToFormatElement for TsBooleanLiteralType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.literal().format(formatter)
    }
}
