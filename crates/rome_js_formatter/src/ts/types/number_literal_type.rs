use crate::prelude::*;
use rome_js_syntax::TsNumberLiteralType;

impl FormatNode for TsNumberLiteralType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let minus = self.minus_token();
        let literal = self.literal_token().format(formatter)?;
        formatted![formatter, minus, literal]
    }
}
