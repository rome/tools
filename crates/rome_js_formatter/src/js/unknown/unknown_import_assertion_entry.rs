use crate::prelude::*;

use rome_js_syntax::JsUnknownImportAssertionEntry;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsUnknownImportAssertionEntry;

impl FormatNodeRule<JsUnknownImportAssertionEntry> for FormatJsUnknownImportAssertionEntry {
    fn fmt_fields(
        &self,
        node: &JsUnknownImportAssertionEntry,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }

    fn prints_comments(&self, _item: &JsUnknownImportAssertionEntry) -> bool {
        true
    }
}
