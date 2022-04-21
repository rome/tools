use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsBooleanLiteralType;

impl FormatNode for TsBooleanLiteralType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.literal().format(formatter)
    }
}
