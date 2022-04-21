use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsPrivateClassMemberName;
use rome_js_syntax::JsPrivateClassMemberNameFields;

impl FormatNode for JsPrivateClassMemberName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPrivateClassMemberNameFields {
            hash_token,
            id_token,
        } = self.as_fields();

        Ok(format_elements![
            hash_token.format(formatter)?,
            id_token.format(formatter)?,
        ])
    }
}
