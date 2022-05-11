//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsxAnyName;
impl Format for JsxAnyName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsxName(node) => node.format(formatter),
            Self::JsxNamespaceName(node) => node.format(formatter),
        }
    }
}
