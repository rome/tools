use crate::prelude::*;
use rome_js_syntax::TsStringType;

impl FormatNode for TsStringType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.string_token().format(formatter)
    }
}
