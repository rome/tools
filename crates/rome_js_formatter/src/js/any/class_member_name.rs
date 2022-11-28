//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::AnyJsClassMemberName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsClassMemberName;
impl FormatRule<AnyJsClassMemberName> for FormatAnyJsClassMemberName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsClassMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsClassMemberName::JsLiteralMemberName(node) => node.format().fmt(f),
            AnyJsClassMemberName::JsComputedMemberName(node) => node.format().fmt(f),
            AnyJsClassMemberName::JsPrivateClassMemberName(node) => node.format().fmt(f),
        }
    }
}
