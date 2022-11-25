//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsxAnyAttributeName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsxAnyAttributeName;
impl FormatRule<JsxAnyAttributeName> for FormatJsxAnyAttributeName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsxAnyAttributeName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyAttributeName::JsxName(node) => node.format().fmt(f),
            JsxAnyAttributeName::JsxNamespaceName(node) => node.format().fmt(f),
        }
    }
}
