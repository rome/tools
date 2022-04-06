use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::{format_elements, FormatResult};
use rome_js_syntax::{JsxClosingElement, JsxClosingElementFields};

impl FormatNode for JsxClosingElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
