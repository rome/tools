//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyImportAssertionEntry;
impl ToFormatElement for JsAnyImportAssertionEntry {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsImportAssertionEntry(node) => node.format(formatter),
            Self::JsUnknownImportAssertionEntry(node) => node.format(formatter),
        }
    }
}
