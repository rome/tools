use crate::formatter::verbatim_node;
use crate::prelude::*;
use rome_js_syntax::JsxClosingElement;
use rome_rowan::AstNode;

impl FormatNode for JsxClosingElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        verbatim_node(self.syntax()).format(formatter)
    }
}
