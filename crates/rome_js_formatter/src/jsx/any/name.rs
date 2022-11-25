//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::JsxAnyName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsxAnyName;
impl FormatRule<JsxAnyName> for FormatJsxAnyName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsxAnyName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyName::JsxName(node) => node.format().fmt(f),
            JsxAnyName::JsxNamespaceName(node) => node.format().fmt(f),
        }
    }
}
