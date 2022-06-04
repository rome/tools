//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyElementName;
use crate::prelude::*;
use rome_js_syntax::JsxAnyElementName;
impl FormatRule<JsxAnyElementName> for FormatJsxAnyElementName {
    type Context = JsFormatContext;
    fn fmt(node: &JsxAnyElementName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyElementName::JsxName(node) => node.format().fmt(f),
            JsxAnyElementName::JsxReferenceIdentifier(node) => node.format().fmt(f),
            JsxAnyElementName::JsxMemberName(node) => node.format().fmt(f),
            JsxAnyElementName::JsxNamespaceName(node) => node.format().fmt(f),
        }
    }
}
