use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{TsArrayType, TsArrayTypeFields};

impl ToFormatElement for TsArrayType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
