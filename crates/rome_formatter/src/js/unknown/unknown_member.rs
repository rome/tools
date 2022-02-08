use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsUnknownMember;

use rslint_parser::AstNode;

impl ToFormatElement for JsUnknownMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}
