use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsNullLiteralType;

impl FormatNode for TsNullLiteralType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.literal_token().format(formatter)
    }
}
