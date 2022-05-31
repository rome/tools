//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyClassMemberName;
use crate::prelude::*;
use rome_js_syntax::JsAnyClassMemberName;
impl FormatRule<JsAnyClassMemberName> for FormatJsAnyClassMemberName {
    type Context = JsFormatContext;
    fn format(node: &JsAnyClassMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyClassMemberName::JsLiteralMemberName(node) => node.format().format(f),
            JsAnyClassMemberName::JsComputedMemberName(node) => node.format().format(f),
            JsAnyClassMemberName::JsPrivateClassMemberName(node) => node.format().format(f),
        }
    }
}
