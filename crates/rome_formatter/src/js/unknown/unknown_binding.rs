use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsUnknownBinding;

use rslint_parser::AstNode;

impl ToFormatElement for JsUnknownBinding {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}
