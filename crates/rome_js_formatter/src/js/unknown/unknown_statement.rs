use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use crate::formatter::unknown_node;
use rome_js_syntax::JsUnknownStatement;
use rome_rowan::AstNode;

impl FormatNode for JsUnknownStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        unknown_node(self.syntax()).format(formatter)
    }
}
