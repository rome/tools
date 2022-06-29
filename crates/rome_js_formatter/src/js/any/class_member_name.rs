//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyClassMemberName;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyClassMemberName;
impl FormatRule<JsAnyClassMemberName> for FormatJsAnyClassMemberName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyClassMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyClassMemberName::JsLiteralMemberName(node) => node.format().fmt(f),
            JsAnyClassMemberName::JsComputedMemberName(node) => node.format().fmt(f),
            JsAnyClassMemberName::JsPrivateClassMemberName(node) => node.format().fmt(f),
        }
    }
}
