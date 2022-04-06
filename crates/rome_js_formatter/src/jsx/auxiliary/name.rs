use crate::{FormatElement, FormatResult, Formatter, ToFormatElement, formatter_traits::FormatTokenAndNode};
use rome_js_syntax::{AstNode, JsxName, JsxNameFields};
impl ToFormatElement for JsxName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxNameFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
