use crate::formatter::verbatim_node;
use crate::prelude::*;
use rome_js_syntax::JsxChildList;
use rome_rowan::AstNode;

impl Format for JsxChildList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        verbatim_node(self.syntax()).format(formatter)
    }
}
