use crate::prelude::*;
use rome_js_syntax::TsDefiniteVariableAnnotation;
use rome_js_syntax::TsDefiniteVariableAnnotationFields;

impl FormatNode for TsDefiniteVariableAnnotation {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsDefiniteVariableAnnotationFields {
            excl_token,
            type_annotation,
        } = self.as_fields();

        formatted![
            formatter,
            excl_token.format(formatter)?,
            type_annotation.format(formatter)?,
        ]
    }
}
