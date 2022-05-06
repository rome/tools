use crate::{formatted, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsPrivateName;
use rome_js_syntax::JsPrivateNameFields;

impl FormatNode for JsPrivateName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPrivateNameFields {
            hash_token,
            value_token,
        } = self.as_fields();

        formatted![
            formatter,
            hash_token.format(formatter)?,
            value_token.format(formatter)?
        ]
    }
}
