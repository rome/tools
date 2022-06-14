use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsxOpeningElement;
use rome_rowan::AstNode;

impl FormatNodeFields<JsxOpeningElement> for FormatNodeRule<JsxOpeningElement> {
    fn fmt_fields(node: &JsxOpeningElement, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(formatter)
    }
}
