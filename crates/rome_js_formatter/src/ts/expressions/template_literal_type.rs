use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsTemplateLiteralType;
use rome_js_syntax::TsTemplateLiteralTypeFields;

impl FormatNode for TsTemplateLiteralType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTemplateLiteralTypeFields {
            l_tick_token,
            elements,
            r_tick_token,
        } = self.as_fields();

        Ok(format_elements![
            l_tick_token.format(formatter)?,
            elements.format(formatter)?,
            r_tick_token.format(formatter)?,
        ])
    }
}
