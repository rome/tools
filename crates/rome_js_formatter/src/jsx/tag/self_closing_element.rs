use crate::format_traits::FormatOptional;
use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsxSelfClosingElement;

impl FormatNode for JsxSelfClosingElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let type_arguments = self.type_arguments().format_or_empty(formatter)?;

        Ok(format_elements![
            self.l_angle_token().format(formatter)?,
            self.name().format(formatter)?,
            type_arguments,
            space_token(),
            self.attributes().format(formatter)?,
            space_token(),
            self.slash_token().format(formatter)?,
            self.r_angle_token().format(formatter)?
        ])
    }
}
