//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyAttributeName;
use crate::prelude::*;
use rome_js_syntax::JsxAnyAttributeName;
impl FormatRule<JsxAnyAttributeName> for FormatJsxAnyAttributeName {
    type Context = JsFormatContext;
    fn fmt(node: &JsxAnyAttributeName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyAttributeName::JsxName(node) => node.format().fmt(f),
            JsxAnyAttributeName::JsxNamespaceName(node) => node.format().fmt(f),
        }
    }
}
