use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsUnknownType;

impl FormatNode for TsUnknownType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.unknown_token().format(formatter)
    }
}
