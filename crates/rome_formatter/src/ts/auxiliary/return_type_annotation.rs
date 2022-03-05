use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_syntax::TsReturnTypeAnnotation;
use rslint_syntax::TsReturnTypeAnnotationFields;

impl ToFormatElement for TsReturnTypeAnnotation {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsReturnTypeAnnotationFields { colon_token, ty } = self.as_fields();
        Ok(format_elements![
            colon_token.format(formatter)?,
            space_token(),
            ty.format(formatter)?
        ])
    }
}
