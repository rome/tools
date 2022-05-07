use crate::prelude::*;
use rome_js_syntax::TsThisParameter;

impl FormatNode for TsThisParameter {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let this = self.this_token().format(formatter)?;
        formatted![formatter, this, self.type_annotation()]
    }
}
