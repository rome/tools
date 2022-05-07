use crate::prelude::*;
use rome_js_syntax::TsVoidType;

impl FormatNode for TsVoidType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.void_token().format(formatter)
    }
}
