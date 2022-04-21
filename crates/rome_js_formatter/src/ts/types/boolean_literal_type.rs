use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsBooleanLiteralType;

impl FormatNode for TsBooleanLiteralType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.literal().format(formatter)
    }
}
