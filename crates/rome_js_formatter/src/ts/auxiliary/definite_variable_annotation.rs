use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsDefiniteVariableAnnotation;
use rome_js_syntax::TsDefiniteVariableAnnotationFields;

impl ToFormatElement for TsDefiniteVariableAnnotation {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsDefiniteVariableAnnotationFields {
            excl_token,
            type_annotation,
        } = self.as_fields();

        Ok(format_elements![
            excl_token.format(formatter)?,
            type_annotation.format(formatter)?,
        ])
    }
}
