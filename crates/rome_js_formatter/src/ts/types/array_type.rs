use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::{TsArrayType, TsArrayTypeFields};

impl FormatNode for TsArrayType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsArrayTypeFields {
            l_brack_token,
            element_type,
            r_brack_token,
        } = self.as_fields();
        Ok(format_elements![
            element_type.format(formatter)?,
            l_brack_token.format(formatter)?,
            r_brack_token.format(formatter)?,
        ])
    }
}
