//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyClassMemberName;
use crate::prelude::*;
use rome_js_syntax::JsAnyClassMemberName;
impl FormatRule<JsAnyClassMemberName> for FormatJsAnyClassMemberName {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyClassMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyClassMemberName::JsLiteralMemberName(node) => node.format().fmt(f),
            JsAnyClassMemberName::JsComputedMemberName(node) => node.format().fmt(f),
            JsAnyClassMemberName::JsPrivateClassMemberName(node) => node.format().fmt(f),
        }
    }
}
