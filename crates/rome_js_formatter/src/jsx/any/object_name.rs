//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsxAnyObjectName;
#[derive(Debug, Clone, Default)]
pub struct FormatJsxAnyObjectName;
impl FormatRule<JsxAnyObjectName> for FormatJsxAnyObjectName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsxAnyObjectName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyObjectName::JsxReferenceIdentifier(node) => node.format().fmt(f),
            JsxAnyObjectName::JsxMemberName(node) => node.format().fmt(f),
            JsxAnyObjectName::JsxNamespaceName(node) => node.format().fmt(f),
        }
    }
}
