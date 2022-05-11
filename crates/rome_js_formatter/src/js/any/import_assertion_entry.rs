//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyImportAssertionEntry;
impl Format for JsAnyImportAssertionEntry {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsImportAssertionEntry(node) => node.format(formatter),
            Self::JsUnknownImportAssertionEntry(node) => node.format(formatter),
        }
    }
}
