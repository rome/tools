use crate::formatter::verbatim_node;
use crate::prelude::*;
use rome_js_syntax::JsxOpeningElement;
use rome_rowan::AstNode;

impl FormatNode for JsxOpeningElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        verbatim_node(self.syntax()).format(formatter)
    }
}
