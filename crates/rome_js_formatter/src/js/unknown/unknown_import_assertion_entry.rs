use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsUnknownImportAssertionEntry;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknownImportAssertionEntry>
    for FormatNodeRule<JsUnknownImportAssertionEntry>
{
    fn fmt_fields(
        node: &JsUnknownImportAssertionEntry,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }
}
