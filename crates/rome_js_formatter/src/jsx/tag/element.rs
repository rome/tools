use crate::formatter::verbatim_node;
use crate::prelude::*;
use rome_js_syntax::JsxElement;
use rome_rowan::AstNode;

impl FormatNode for JsxElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        verbatim_node(self.syntax()).format(formatter)
    }
}
