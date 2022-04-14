use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::{format_elements, FormatResult};
use rome_js_syntax::{JsxSpreadAttribute, JsxSpreadAttributeFields};

impl FormatNode for JsxSpreadAttribute {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxSpreadAttributeFields {
            l_curly_token,
            dotdotdot_token,
            argument,
            r_curly_token,
        } = self.as_fields();

        Ok(format_elements![
            l_curly_token.format(formatter)?,
            dotdotdot_token.format(formatter)?,
            argument.format(formatter)?,
            r_curly_token.format(formatter)?,
        ])
    }
}
