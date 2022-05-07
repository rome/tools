use crate::prelude::*;

use rome_js_syntax::JsObjectBindingPattern;
use rome_js_syntax::JsObjectBindingPatternFields;

impl FormatNode for JsObjectBindingPattern {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectBindingPatternFields {
            l_curly_token,
            properties,
            r_curly_token,
        } = self.as_fields();

        formatter.format_delimited_soft_block_spaces(
            &l_curly_token?,
            properties.format(formatter)?,
            &r_curly_token?,
        )
    }
}
