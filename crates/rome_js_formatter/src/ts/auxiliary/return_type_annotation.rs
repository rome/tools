use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
use rome_js_syntax::TsReturnTypeAnnotation;
use rome_js_syntax::TsReturnTypeAnnotationFields;

impl FormatNode for TsReturnTypeAnnotation {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsReturnTypeAnnotationFields { colon_token, ty } = self.as_fields();
        Ok(format_elements![
            colon_token.format(formatter)?,
            space_token(),
            ty.format(formatter)?
        ])
    }
}
