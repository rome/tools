use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
use rome_js_syntax::TsTypeAnnotation;

impl FormatNode for TsTypeAnnotation {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let colon = self.colon_token().format(formatter)?;
        let ty = self.ty().format(formatter)?;

        Ok(format_elements![colon, space_token(), ty])
    }
}
