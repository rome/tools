use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsBigIntLiteralType;

impl ToFormatElement for TsBigIntLiteralType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let minus = self.minus_token().format_or_empty(formatter)?;
        let literal = self.literal_token().format(formatter)?;
        Ok(format_elements![minus, literal])
    }
}
