use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsTypeAnnotation, TsTypeAnnotationFields};

impl FormatNodeFields<TsTypeAnnotation> for FormatNodeRule<TsTypeAnnotation> {
    fn fmt_fields(node: &TsTypeAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeAnnotationFields { colon_token, ty } = node.as_fields();
        let colon = colon_token.format();
        let ty = ty.format();

        write![f, [colon, space_token(), ty]]
    }
}
