use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::{ast::TsArrayType, AstNode};

impl ToFormatElement for TsArrayType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.l_brack_token().format(formatter)?,
            self.element_type().format(formatter)?,
            self.r_brack_token().format(formatter)?,
        ])
    }
}
