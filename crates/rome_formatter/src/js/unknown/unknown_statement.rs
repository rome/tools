use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsUnknownStatement;

use rslint_parser::AstNode;

impl ToFormatElement for JsUnknownStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_unknown(self.syntax()))
    }
}
