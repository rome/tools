use crate::prelude::*;
use rome_js_syntax::TsThisType;

impl FormatNode for TsThisType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.this_token().format(formatter)
    }
}
