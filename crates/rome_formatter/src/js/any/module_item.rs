//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyModuleItem;
impl ToFormatElement for JsAnyModuleItem {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyStatement(node) => node.to_format_element(formatter),
            Self::JsExport(node) => node.to_format_element(formatter),
            Self::JsImport(node) => node.to_format_element(formatter),
        }
    }
}
