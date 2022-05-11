//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyName;
use crate::prelude::*;
use rome_js_syntax::JsxAnyName;
impl FormatRule<JsxAnyName> for FormatJsxAnyName {
    fn format(node: &JsxAnyName, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsxAnyName::JsxName(node) => formatted![formatter, [node.format()]],
            JsxAnyName::JsxNamespaceName(node) => formatted![formatter, [node.format()]],
        }
    }
}
