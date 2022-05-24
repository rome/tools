//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyClassMemberName;
use crate::prelude::*;
use rome_js_syntax::JsAnyClassMemberName;
impl FormatRule<JsAnyClassMemberName> for FormatJsAnyClassMemberName {
    type Options = JsFormatOptions;
    fn format(
        node: &JsAnyClassMemberName,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyClassMemberName::JsLiteralMemberName(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyClassMemberName::JsComputedMemberName(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyClassMemberName::JsPrivateClassMemberName(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
