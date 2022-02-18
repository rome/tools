use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsTupleType;

impl ToFormatElement for TsTupleType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_delimited_soft_block_indent(
            &self.l_brack_token()?,
            self.elements().to_format_element(formatter)?,
            &self.r_brack_token()?,
        )
    }
}
