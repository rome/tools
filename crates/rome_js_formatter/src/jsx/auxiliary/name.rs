use crate::{
    formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::{AstNode, JsxName, JsxNameFields};
impl ToFormatElement for JsxName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxNameFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
