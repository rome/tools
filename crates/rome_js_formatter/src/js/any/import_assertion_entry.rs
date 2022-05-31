//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyImportAssertionEntry;
use crate::prelude::*;
use rome_js_syntax::JsAnyImportAssertionEntry;
impl FormatRule<JsAnyImportAssertionEntry> for FormatJsAnyImportAssertionEntry {
    type Context = JsFormatContext;
    fn format(
        node: &JsAnyImportAssertionEntry,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyImportAssertionEntry::JsImportAssertionEntry(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyImportAssertionEntry::JsUnknownImportAssertionEntry(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
