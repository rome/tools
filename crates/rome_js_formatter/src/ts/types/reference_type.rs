use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsReferenceType;

impl ToFormatElement for TsReferenceType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = self.name().format(formatter)?;
        let type_arguments = self.type_arguments().format_or_empty(formatter)?;
        Ok(format_elements![name, type_arguments])
    }
}
