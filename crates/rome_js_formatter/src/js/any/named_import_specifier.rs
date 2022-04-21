//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnyNamedImportSpecifier;
impl Format for JsAnyNamedImportSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsShorthandNamedImportSpecifier(node) => node.format(formatter),
            Self::JsNamedImportSpecifier(node) => node.format(formatter),
            Self::JsUnknownNamedImportSpecifier(node) => node.format(formatter),
        }
    }
}
