use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsUnknownNamedImportSpecifier;
use rome_rowan::AstNode;

impl ToFormatElement for JsUnknownNamedImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_unknown(self.syntax()))
    }
}
