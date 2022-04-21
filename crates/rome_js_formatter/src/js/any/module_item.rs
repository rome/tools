//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnyModuleItem;
impl Format for JsAnyModuleItem {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyStatement(node) => node.format(formatter),
            Self::JsExport(node) => node.format(formatter),
            Self::JsImport(node) => node.format(formatter),
        }
    }
}
