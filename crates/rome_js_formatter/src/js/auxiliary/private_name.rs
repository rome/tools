use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsPrivateName;
use rome_js_syntax::JsPrivateNameFields;

impl FormatNode for JsPrivateName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
