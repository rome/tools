//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyElementName;
use crate::prelude::*;
use rome_js_syntax::JsxAnyElementName;
impl FormatRule<JsxAnyElementName> for FormatJsxAnyElementName {
    type Options = JsFormatOptions;
    fn format(
        node: &JsxAnyElementName,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsxAnyElementName::JsxName(node) => formatted![formatter, [node.format()]],
            JsxAnyElementName::JsxReferenceIdentifier(node) => {
                formatted![formatter, [node.format()]]
            }
            JsxAnyElementName::JsxMemberName(node) => formatted![formatter, [node.format()]],
            JsxAnyElementName::JsxNamespaceName(node) => formatted![formatter, [node.format()]],
        }
    }
}
