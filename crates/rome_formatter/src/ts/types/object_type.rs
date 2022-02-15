use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsObjectType;

impl ToFormatElement for TsObjectType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_delimited_soft_block_spaces(
            &self.l_curly_token()?,
            self.members().to_format_element(formatter)?,
            &self.r_curly_token()?,
        )
    }
}
