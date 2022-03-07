use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsOptionalPropertyAnnotation;
use rome_js_syntax::TsOptionalPropertyAnnotationFields;

impl ToFormatElement for TsOptionalPropertyAnnotation {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsOptionalPropertyAnnotationFields {
            question_mark_token,
            type_annotation,
        } = self.as_fields();

        Ok(format_elements![
            question_mark_token.format(formatter)?,
            type_annotation.format_or_empty(formatter)?
        ])
    }
}
