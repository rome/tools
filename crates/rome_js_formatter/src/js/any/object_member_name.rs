//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyObjectMemberName;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyObjectMemberName;
impl FormatRule<JsAnyObjectMemberName> for FormatJsAnyObjectMemberName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyObjectMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyObjectMemberName::JsLiteralMemberName(node) => node.format().fmt(f),
            JsAnyObjectMemberName::JsComputedMemberName(node) => node.format().fmt(f),
        }
    }
}
