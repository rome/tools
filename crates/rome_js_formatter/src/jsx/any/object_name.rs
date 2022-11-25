//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::JsxAnyObjectName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsxAnyObjectName;
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
