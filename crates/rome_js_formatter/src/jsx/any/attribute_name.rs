//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsxAnyAttributeName;
impl Format for JsxAnyAttributeName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsxName(node) => node.format(formatter),
            Self::JsxNamespaceName(node) => node.format(formatter),
        }
    }
}
