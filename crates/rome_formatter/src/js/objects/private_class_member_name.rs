use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsPrivateClassMemberName;
use rome_js_syntax::JsPrivateClassMemberNameFields;

impl ToFormatElement for JsPrivateClassMemberName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
