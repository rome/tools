use crate::prelude::*;

use rome_js_syntax::JsUnknownParameter;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsUnknownParameter;

impl FormatNodeRule<JsUnknownParameter> for FormatJsUnknownParameter {
    fn fmt_fields(
        &self,
        node: &JsUnknownParameter,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }

    fn prints_comments(&self, _item: &JsUnknownParameter) -> bool {
        true
    }
}
