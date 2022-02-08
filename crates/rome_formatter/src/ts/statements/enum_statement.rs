use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::{ast::TsEnumStatement, AstNode};
impl ToFormatElement for TsEnumStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}