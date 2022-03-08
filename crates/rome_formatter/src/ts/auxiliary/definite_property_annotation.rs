use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsDefinitePropertyAnnotation;
use rome_js_syntax::TsDefinitePropertyAnnotationFields;

impl ToFormatElement for TsDefinitePropertyAnnotation {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsDefinitePropertyAnnotationFields {
            excl_token,
            type_annotation,
        } = self.as_fields();
        Ok(format_elements![
            excl_token.format(formatter)?,
            type_annotation.format(formatter)?
        ])
    }
}
