use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsBigintType;

impl ToFormatElement for TsBigintType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.bigint_token().format(formatter)
    }
}
