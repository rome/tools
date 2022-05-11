//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyAttributeName;
use crate::prelude::*;
use rome_js_syntax::JsxAnyAttributeName;
impl FormatRule<JsxAnyAttributeName> for FormatJsxAnyAttributeName {
    fn format(node: &JsxAnyAttributeName, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsxAnyAttributeName::JsxName(node) => formatted![formatter, [node.format()]],
            JsxAnyAttributeName::JsxNamespaceName(node) => formatted![formatter, [node.format()]],
        }
    }
}
