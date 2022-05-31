use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::TsReturnTypeAnnotation;
use rome_js_syntax::TsReturnTypeAnnotationFields;

impl FormatNodeFields<TsReturnTypeAnnotation> for FormatNodeRule<TsReturnTypeAnnotation> {
    fn format_fields(node: &TsReturnTypeAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        let TsReturnTypeAnnotationFields { colon_token, ty } = node.as_fields();
        write![f, [colon_token.format(), space_token(), ty.format()]]
    }
}
