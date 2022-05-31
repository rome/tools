use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsTypeAnnotation, TsTypeAnnotationFields};

impl FormatNodeFields<TsTypeAnnotation> for FormatNodeRule<TsTypeAnnotation> {
    fn format_fields(
        node: &TsTypeAnnotation,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsTypeAnnotationFields { colon_token, ty } = node.as_fields();
        let colon = colon_token.format();
        let ty = ty.format();

        formatted![formatter, [colon, space_token(), ty]]
    }
}
