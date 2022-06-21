use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsOptionalPropertyAnnotation;
use rome_js_syntax::TsOptionalPropertyAnnotationFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsOptionalPropertyAnnotation;

impl FormatNodeRule<TsOptionalPropertyAnnotation> for FormatTsOptionalPropertyAnnotation {
    fn fmt_fields(
        &self,
        node: &TsOptionalPropertyAnnotation,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsOptionalPropertyAnnotationFields {
            question_mark_token,
            type_annotation,
        } = node.as_fields();

        write![f, [question_mark_token.format(), type_annotation.format()]]
    }
}
