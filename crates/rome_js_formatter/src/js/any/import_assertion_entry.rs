//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyImportAssertionEntry;
use crate::prelude::*;
use rome_js_syntax::JsAnyImportAssertionEntry;
impl FormatRule<JsAnyImportAssertionEntry> for FormatJsAnyImportAssertionEntry {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyImportAssertionEntry, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyImportAssertionEntry::JsImportAssertionEntry(node) => node.format().fmt(f),
            JsAnyImportAssertionEntry::JsUnknownImportAssertionEntry(node) => node.format().fmt(f),
        }
    }
}
