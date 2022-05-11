use crate::prelude::*;
use rome_js_syntax::TsBigintType;

impl FormatNode for TsBigintType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.bigint_token().format(formatter)
    }
}
