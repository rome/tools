use crate::prelude::*;
use rome_js_syntax::TsBigIntLiteralType;

impl FormatNode for TsBigIntLiteralType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let literal = self.literal_token().format(formatter)?;
        formatted![formatter, self.minus_token(), literal]
    }
}
