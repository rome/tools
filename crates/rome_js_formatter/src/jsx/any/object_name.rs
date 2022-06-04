//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyObjectName;
use crate::prelude::*;
use rome_js_syntax::JsxAnyObjectName;
impl FormatRule<JsxAnyObjectName> for FormatJsxAnyObjectName {
    type Context = JsFormatContext;
    fn fmt(node: &JsxAnyObjectName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyObjectName::JsxReferenceIdentifier(node) => node.format().fmt(f),
            JsxAnyObjectName::JsxMemberName(node) => node.format().fmt(f),
            JsxAnyObjectName::JsxNamespaceName(node) => node.format().fmt(f),
        }
    }
}
