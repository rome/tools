//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::JsxAnyObjectName;
impl Format for JsxAnyObjectName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsxReferenceIdentifier(node) => node.format(formatter),
            Self::JsxMemberName(node) => node.format(formatter),
            Self::JsxNamespaceName(node) => node.format(formatter),
        }
    }
}
