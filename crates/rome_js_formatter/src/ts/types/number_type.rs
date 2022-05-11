use crate::prelude::*;
use rome_js_syntax::TsNumberType;

impl FormatNode for TsNumberType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.number_token().format(formatter)
    }
}
