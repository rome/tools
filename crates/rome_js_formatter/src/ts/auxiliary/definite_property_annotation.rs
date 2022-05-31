use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsDefinitePropertyAnnotation;
use rome_js_syntax::TsDefinitePropertyAnnotationFields;

impl FormatNodeFields<TsDefinitePropertyAnnotation>
    for FormatNodeRule<TsDefinitePropertyAnnotation>
{
    fn format_fields(
        node: &TsDefinitePropertyAnnotation,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsDefinitePropertyAnnotationFields {
            excl_token,
            type_annotation,
        } = node.as_fields();
        formatted![formatter, [excl_token.format(), type_annotation.format()]]
    }
}
