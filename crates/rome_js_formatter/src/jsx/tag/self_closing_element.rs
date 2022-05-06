use crate::{formatted, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsxSelfClosingElement;

impl FormatNode for JsxSelfClosingElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatted![
            formatter,
            self.l_angle_token().format(formatter)?,
            self.name().format(formatter)?,
            self.type_arguments(),
            space_token(),
            self.attributes().format(formatter)?,
            space_token(),
            self.slash_token().format(formatter)?,
            self.r_angle_token().format(formatter)?
        ]
    }
}
