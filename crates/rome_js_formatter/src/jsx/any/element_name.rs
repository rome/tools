//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsxAnyElementName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsxAnyElementName;
impl FormatRule<JsxAnyElementName> for FormatJsxAnyElementName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsxAnyElementName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyElementName::JsxName(node) => node.format().fmt(f),
            JsxAnyElementName::JsxReferenceIdentifier(node) => node.format().fmt(f),
            JsxAnyElementName::JsxMemberName(node) => node.format().fmt(f),
            JsxAnyElementName::JsxNamespaceName(node) => node.format().fmt(f),
        }
    }
}
