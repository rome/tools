use crate::{formatted, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsSpread;
use rome_js_syntax::JsSpreadFields;

impl FormatNode for JsSpread {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsSpreadFields {
            dotdotdot_token,
            argument,
        } = self.as_fields();

        formatted![
            formatter,
            dotdotdot_token.format(formatter)?,
            argument.format(formatter)?
        ]
    }
}
