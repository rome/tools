use crate::prelude::*;
use rome_js_syntax::TsUndefinedType;

impl FormatNode for TsUndefinedType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.undefined_token().format(formatter)
    }
}
