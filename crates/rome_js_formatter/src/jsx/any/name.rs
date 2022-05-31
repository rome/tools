//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyName;
use crate::prelude::*;
use rome_js_syntax::JsxAnyName;
impl FormatRule<JsxAnyName> for FormatJsxAnyName {
    type Context = JsFormatContext;
    fn format(node: &JsxAnyName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyName::JsxName(node) => node.format().format(f),
            JsxAnyName::JsxNamespaceName(node) => node.format().format(f),
        }
    }
}
