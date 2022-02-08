use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::{ast::TsIndexSignatureObjectTypeMember, AstNode};
impl ToFormatElement for TsIndexSignatureObjectTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}