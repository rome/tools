use crate::{FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsUnknownNamedImportSpecifier;
use rome_rowan::AstNode;

impl FormatNode for JsUnknownNamedImportSpecifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_unknown(self.syntax()))
    }
}
