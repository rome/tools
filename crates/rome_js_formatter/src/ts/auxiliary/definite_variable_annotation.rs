use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsDefiniteVariableAnnotation;
use rome_js_syntax::TsDefiniteVariableAnnotationFields;

impl FormatNodeFields<TsDefiniteVariableAnnotation>
    for FormatNodeRule<TsDefiniteVariableAnnotation>
{
    fn format_fields(
        node: &TsDefiniteVariableAnnotation,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsDefiniteVariableAnnotationFields {
            excl_token,
            type_annotation,
        } = node.as_fields();

        formatted![formatter, [excl_token.format(), type_annotation.format(),]]
    }
}
