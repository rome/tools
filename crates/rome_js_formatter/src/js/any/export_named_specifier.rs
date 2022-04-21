//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnyExportNamedSpecifier;
impl Format for JsAnyExportNamedSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsExportNamedShorthandSpecifier(node) => node.format(formatter),
            Self::JsExportNamedSpecifier(node) => node.format(formatter),
        }
    }
}
