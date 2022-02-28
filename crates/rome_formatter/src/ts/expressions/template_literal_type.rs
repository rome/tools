use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsTemplateLiteralType;
use rslint_parser::ast::TsTemplateLiteralTypeFields;

impl ToFormatElement for TsTemplateLiteralType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
