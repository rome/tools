//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyClassMemberName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyClassMemberName;
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
