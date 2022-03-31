use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsSpread;
use rome_js_syntax::JsSpreadFields;

impl ToFormatElement for JsSpread {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
