use crate::prelude::*;

use rome_js_syntax::JsArrayBindingPatternRestElement;
use rome_js_syntax::JsArrayBindingPatternRestElementFields;

impl FormatNode for JsArrayBindingPatternRestElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsArrayBindingPatternRestElementFields {
            dotdotdot_token,
            pattern,
        } = self.as_fields();

        formatted![
            formatter,
            dotdotdot_token.format(formatter)?,
            pattern.format(formatter)?,
        ]
    }
}
