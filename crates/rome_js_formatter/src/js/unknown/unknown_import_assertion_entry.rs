use crate::prelude::*;

use crate::formatter::unknown_node;
use crate::FormatNodeFields;
use rome_js_syntax::JsUnknownImportAssertionEntry;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknownImportAssertionEntry>
    for FormatNodeRule<JsUnknownImportAssertionEntry>
{
    fn format_fields(
        node: &JsUnknownImportAssertionEntry,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        unknown_node(node.syntax()).format(formatter)
    }
}
