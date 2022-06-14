use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsOptionalPropertyAnnotation;
use rome_js_syntax::TsOptionalPropertyAnnotationFields;

impl FormatNodeFields<TsOptionalPropertyAnnotation>
    for FormatNodeRule<TsOptionalPropertyAnnotation>
{
    fn fmt_fields(node: &TsOptionalPropertyAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        let TsOptionalPropertyAnnotationFields {
            question_mark_token,
            type_annotation,
        } = node.as_fields();

        write![f, [question_mark_token.format(), type_annotation.format()]]
    }
}
