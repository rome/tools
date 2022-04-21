use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsSpread;
use rome_js_syntax::JsSpreadFields;

impl FormatNode for JsSpread {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsSpreadFields {
            dotdotdot_token,
            argument,
        } = self.as_fields();

        Ok(format_elements![
            dotdotdot_token.format(formatter)?,
            argument.format(formatter)?
        ])
    }
}
