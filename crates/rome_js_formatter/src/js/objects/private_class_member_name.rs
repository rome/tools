use crate::{formatted, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsPrivateClassMemberName;
use rome_js_syntax::JsPrivateClassMemberNameFields;

impl FormatNode for JsPrivateClassMemberName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPrivateClassMemberNameFields {
            hash_token,
            id_token,
        } = self.as_fields();

        formatted![
            formatter,
            hash_token.format(formatter)?,
            id_token.format(formatter)?,
        ]
    }
}
