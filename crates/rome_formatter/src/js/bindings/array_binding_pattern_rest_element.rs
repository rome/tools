use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsArrayBindingPatternRestElement;
use rslint_syntax::JsArrayBindingPatternRestElementFields;

impl ToFormatElement for JsArrayBindingPatternRestElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsArrayBindingPatternRestElementFields {
            dotdotdot_token,
            pattern,
        } = self.as_fields();

        Ok(format_elements![
            dotdotdot_token.format(formatter)?,
            pattern.format(formatter)?,
        ])
    }
}
