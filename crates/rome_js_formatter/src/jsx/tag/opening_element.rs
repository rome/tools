use crate::{space_token, Format, FormatElement, FormatNode, Formatter, JsFormatter};
use rome_formatter::{empty_element, format_elements, FormatResult};
use rome_js_syntax::{JsxOpeningElement, JsxOpeningElementFields};
use rome_rowan::AstNodeList;

impl FormatNode for JsxOpeningElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxOpeningElementFields {
            type_arguments: _,
            l_angle_token,
            name,
            attributes,
            r_angle_token,
        } = self.as_fields();

        let optional_space = if attributes.is_empty() {
            empty_element()
        } else {
            space_token()
        };

        Ok(format_elements![
            l_angle_token.format(formatter)?,
            name.format(formatter)?,
            optional_space,
            formatter.format_list(attributes),
            r_angle_token.format(formatter)?
        ])
    }
}
