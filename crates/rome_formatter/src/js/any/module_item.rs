//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyModuleItem;
impl ToFormatElement for JsAnyModuleItem {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyStatement(node) => node.format(formatter),
            Self::JsExport(node) => node.format(formatter),
            Self::JsImport(node) => node.format(formatter),
        }
    }
}
