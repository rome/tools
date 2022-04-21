use crate::format_traits::FormatOptional;
use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsBigIntLiteralType;

impl FormatNode for TsBigIntLiteralType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let minus = self.minus_token().format_or_empty(formatter)?;
        let literal = self.literal_token().format(formatter)?;
        Ok(format_elements![minus, literal])
    }
}
