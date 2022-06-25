//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsxAnyAttributeName;
#[derive(Debug, Clone, Default)]
pub struct FormatJsxAnyAttributeName;
impl FormatRule<JsxAnyAttributeName> for FormatJsxAnyAttributeName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsxAnyAttributeName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyAttributeName::JsxName(node) => node.format().fmt(f),
            JsxAnyAttributeName::JsxNamespaceName(node) => node.format().fmt(f),
        }
    }
}
