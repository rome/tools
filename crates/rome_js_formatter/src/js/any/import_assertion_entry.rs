//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::JsAnyImportAssertionEntry;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyImportAssertionEntry;
impl FormatRule<JsAnyImportAssertionEntry> for FormatJsAnyImportAssertionEntry {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyImportAssertionEntry, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyImportAssertionEntry::JsImportAssertionEntry(node) => node.format().fmt(f),
            JsAnyImportAssertionEntry::JsUnknownImportAssertionEntry(node) => node.format().fmt(f),
        }
    }
}
