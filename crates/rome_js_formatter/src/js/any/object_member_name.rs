//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyObjectMemberName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyObjectMemberName;
impl FormatRule<JsAnyObjectMemberName> for FormatJsAnyObjectMemberName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyObjectMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyObjectMemberName::JsLiteralMemberName(node) => node.format().fmt(f),
            JsAnyObjectMemberName::JsComputedMemberName(node) => node.format().fmt(f),
        }
    }
}
