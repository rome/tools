use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsNeverType;

impl ToFormatElement for TsNeverType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.never_token().format(formatter)
    }
}
