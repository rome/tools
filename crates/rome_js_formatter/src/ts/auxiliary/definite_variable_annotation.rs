use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsDefiniteVariableAnnotation;
use rome_js_syntax::TsDefiniteVariableAnnotationFields;

impl FormatNodeFields<TsDefiniteVariableAnnotation>
    for FormatNodeRule<TsDefiniteVariableAnnotation>
{
    fn fmt_fields(node: &TsDefiniteVariableAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        let TsDefiniteVariableAnnotationFields {
            excl_token,
            type_annotation,
        } = node.as_fields();

        write![f, [excl_token.format(), type_annotation.format()]]
    }
}
