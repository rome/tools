use crate::{space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsTypeAnnotation;

impl FormatNode for TsTypeAnnotation {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let colon = self.colon_token().format(formatter)?;
        let ty = self.ty().format(formatter)?;

        formatted![formatter, colon, space_token(), ty]
    }
}
