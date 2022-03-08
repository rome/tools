use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsPrivateName;
use rome_js_syntax::JsPrivateNameFields;

impl ToFormatElement for JsPrivateName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPrivateNameFields {
            hash_token,
            value_token,
        } = self.as_fields();

        Ok(format_elements![
            hash_token.format(formatter)?,
            value_token.format(formatter)?
        ])
    }
}
