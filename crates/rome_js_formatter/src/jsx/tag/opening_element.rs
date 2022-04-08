use crate::{
    formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_formatter::format_elements;
use rome_js_syntax::{JsxOpeningElement, JsxOpeningElementFields};
use rome_rowan::AstNode;

impl ToFormatElement for JsxOpeningElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxOpeningElementFields {
            type_arguments,
            l_angle_token,
            name,
            attributes,
            r_angle_token,
        } = self.as_fields();

        Ok(format_elements![
            l_angle_token.format(formatter)?,
            name.format(formatter)?,
            formatter.format_list(attributes),
            r_angle_token.format(formatter)?
        ])
    }
}
