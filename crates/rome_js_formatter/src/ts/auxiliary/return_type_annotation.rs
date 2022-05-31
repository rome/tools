use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsReturnTypeAnnotation;
use rome_js_syntax::TsReturnTypeAnnotationFields;

impl FormatNodeFields<TsReturnTypeAnnotation> for FormatNodeRule<TsReturnTypeAnnotation> {
    fn format_fields(
        node: &TsReturnTypeAnnotation,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsReturnTypeAnnotationFields { colon_token, ty } = node.as_fields();
        formatted![
            formatter,
            [colon_token.format(), space_token(), ty.format()]
        ]
    }
}
