use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsDefinitePropertyAnnotation;
use rome_js_syntax::TsDefinitePropertyAnnotationFields;

impl FormatNodeFields<TsDefinitePropertyAnnotation>
    for FormatNodeRule<TsDefinitePropertyAnnotation>
{
    fn fmt_fields(node: &TsDefinitePropertyAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        let TsDefinitePropertyAnnotationFields {
            excl_token,
            type_annotation,
        } = node.as_fields();

        write![f, [excl_token.format(), type_annotation.format()]]
    }
}
