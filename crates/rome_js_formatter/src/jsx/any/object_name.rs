//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyObjectName;
use crate::prelude::*;
use rome_js_syntax::JsxAnyObjectName;
impl FormatRule<JsxAnyObjectName> for FormatJsxAnyObjectName {
    type Options = JsFormatOptions;
    fn format(
        node: &JsxAnyObjectName,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsxAnyObjectName::JsxReferenceIdentifier(node) => {
                formatted![formatter, [node.format()]]
            }
            JsxAnyObjectName::JsxMemberName(node) => formatted![formatter, [node.format()]],
            JsxAnyObjectName::JsxNamespaceName(node) => formatted![formatter, [node.format()]],
        }
    }
}
