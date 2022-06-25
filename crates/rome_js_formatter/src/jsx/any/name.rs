//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsxAnyName;
#[derive(Debug, Clone, Default)]
pub struct FormatJsxAnyName;
impl FormatRule<JsxAnyName> for FormatJsxAnyName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsxAnyName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyName::JsxName(node) => node.format().fmt(f),
            JsxAnyName::JsxNamespaceName(node) => node.format().fmt(f),
        }
    }
}
