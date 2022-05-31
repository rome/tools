//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyElementName;
use crate::prelude::*;
use rome_js_syntax::JsxAnyElementName;
impl FormatRule<JsxAnyElementName> for FormatJsxAnyElementName {
    type Context = JsFormatContext;
    fn format(node: &JsxAnyElementName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyElementName::JsxName(node) => node.format().format(f),
            JsxAnyElementName::JsxReferenceIdentifier(node) => node.format().format(f),
            JsxAnyElementName::JsxMemberName(node) => node.format().format(f),
            JsxAnyElementName::JsxNamespaceName(node) => node.format().format(f),
        }
    }
}
