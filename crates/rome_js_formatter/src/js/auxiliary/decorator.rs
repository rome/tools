use crate::prelude::*;
use rome_js_syntax::JsDecorator;
use rome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsDecorator;
impl FormatNodeRule<JsDecorator> for FormatJsDecorator {
    fn fmt_fields(&self, node: &JsDecorator, f: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
