use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::{ast::TsCallSignatureObjectTypeMember, AstNode};
impl ToFormatElement for TsCallSignatureObjectTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}