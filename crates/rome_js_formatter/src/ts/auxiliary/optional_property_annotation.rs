use crate::format_traits::FormatOptional;
use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsOptionalPropertyAnnotation;
use rome_js_syntax::TsOptionalPropertyAnnotationFields;

impl FormatNode for TsOptionalPropertyAnnotation {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsOptionalPropertyAnnotationFields {
            question_mark_token,
            type_annotation,
        } = self.as_fields();

        formatted![
            formatter,
            question_mark_token.format(formatter)?,
            type_annotation.format_or_empty(formatter)?
        ]
    }
}
