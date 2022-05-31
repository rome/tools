//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyObjectMemberName;
use crate::prelude::*;
use rome_js_syntax::JsAnyObjectMemberName;
impl FormatRule<JsAnyObjectMemberName> for FormatJsAnyObjectMemberName {
    type Context = JsFormatContext;
    fn format(node: &JsAnyObjectMemberName, f: &mut Formatter<Self::Context>) -> FormatResult<()> {
        match node {
            JsAnyObjectMemberName::JsLiteralMemberName(node) => node.format().format(f),
            JsAnyObjectMemberName::JsComputedMemberName(node) => node.format().format(f),
        }
    }
}
