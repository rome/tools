use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsVoidType;

impl FormatNode for TsVoidType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.void_token().format(formatter)
    }
}
