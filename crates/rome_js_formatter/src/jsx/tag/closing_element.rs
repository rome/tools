use crate::{
    formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_formatter::format_elements;
use rome_js_syntax::{AstNode, JsxClosingElement, JsxClosingElementFields};
impl ToFormatElement for JsxClosingElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxClosingElementFields {
            l_angle_token,
            slash_token,
            name,
            r_angle_token,
        } = self.as_fields();

        Ok(format_elements![
            l_angle_token.format(formatter)?,
            slash_token.format(formatter)?,
            name.format(formatter)?,
            r_angle_token.format(formatter)?
        ])
    }
}
