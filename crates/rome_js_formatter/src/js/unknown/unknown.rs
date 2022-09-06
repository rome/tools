use crate::prelude::*;

use rome_js_syntax::JsUnknown;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsUnknown;

impl FormatNodeRule<JsUnknown> for FormatJsUnknown {
    fn fmt_fields(&self, node: &JsUnknown, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }

    fn prints_comments(&self, _item: &JsUnknown) -> bool {
        true
    }
}
