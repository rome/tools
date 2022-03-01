use crate::utils::format_conditional;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsConditionalType;
use rslint_parser::AstNode;

impl ToFormatElement for TsConditionalType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_conditional(self.syntax(), formatter, false)
    }
}
