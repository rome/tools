use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsTypeAnnotation;

impl ToFormatElement for TsTypeAnnotation {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let colon = self.colon_token().format(formatter)?;
        let ty = self.ty().format(formatter)?;

        Ok(format_elements![colon, space_token(), ty])
    }
}
