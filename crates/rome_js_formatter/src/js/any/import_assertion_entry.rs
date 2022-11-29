//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::AnyJsImportAssertionEntry;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsImportAssertionEntry;
impl FormatRule<AnyJsImportAssertionEntry> for FormatAnyJsImportAssertionEntry {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsImportAssertionEntry, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsImportAssertionEntry::JsImportAssertionEntry(node) => node.format().fmt(f),
            AnyJsImportAssertionEntry::JsBogusImportAssertionEntry(node) => node.format().fmt(f),
        }
    }
}
