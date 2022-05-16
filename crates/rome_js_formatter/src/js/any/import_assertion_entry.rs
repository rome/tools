//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyImportAssertionEntry;
use crate::prelude::*;
use rome_js_syntax::JsAnyImportAssertionEntry;
impl FormatRule<JsAnyImportAssertionEntry> for FormatJsAnyImportAssertionEntry {
    type Options = JsFormatOptions;
    fn format(
        node: &JsAnyImportAssertionEntry,
        formatter: &Formatter<Self::Options>,
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
