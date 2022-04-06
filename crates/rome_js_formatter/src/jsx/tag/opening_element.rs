use crate::{Format, FormatElement, FormatNode, Formatter, JsFormatter};
use rome_formatter::{format_elements, FormatResult};
use rome_js_syntax::{JsxOpeningElement, JsxOpeningElementFields};

impl FormatNode for JsxOpeningElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxOpeningElementFields {
            type_arguments: _,
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
