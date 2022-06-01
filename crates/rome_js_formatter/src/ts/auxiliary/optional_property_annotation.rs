use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsOptionalPropertyAnnotation;
use rome_js_syntax::TsOptionalPropertyAnnotationFields;

impl FormatNodeFields<TsOptionalPropertyAnnotation>
    for FormatNodeRule<TsOptionalPropertyAnnotation>
{
    fn format_fields(
        node: &TsOptionalPropertyAnnotation,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsOptionalPropertyAnnotationFields {
            question_mark_token,
            type_annotation,
        } = node.as_fields();

        formatted![
            formatter,
            [question_mark_token.format(), type_annotation.format()]
        ]
    }
}
