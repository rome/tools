use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsUnknownType;

impl ToFormatElement for TsUnknownType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.unknown_token().format(formatter)
    }
}
