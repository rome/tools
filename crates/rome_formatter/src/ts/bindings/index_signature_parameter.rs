use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsIndexSignatureParameter;

impl ToFormatElement for TsIndexSignatureParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let binding = self.binding().format(formatter)?;
        let type_annotation = self.type_annotation().format(formatter)?;

        Ok(format_elements![binding, type_annotation])
    }
}
