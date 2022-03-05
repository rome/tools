use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsObjectBindingPatternRest;
use rslint_syntax::JsObjectBindingPatternRestFields;

impl ToFormatElement for JsObjectBindingPatternRest {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectBindingPatternRestFields {
            dotdotdot_token,
            binding,
        } = self.as_fields();

        Ok(format_elements![
            dotdotdot_token.format(formatter)?,
            binding.format(formatter)?,
        ])
    }
}
