//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyImportAssertionEntry;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyImportAssertionEntry;
impl FormatRule<JsAnyImportAssertionEntry> for FormatJsAnyImportAssertionEntry {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyImportAssertionEntry, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyImportAssertionEntry::JsImportAssertionEntry(node) => node.format().fmt(f),
            JsAnyImportAssertionEntry::JsUnknownImportAssertionEntry(node) => node.format().fmt(f),
        }
    }
}
