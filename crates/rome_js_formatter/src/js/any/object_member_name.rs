//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyObjectMemberName;
use crate::prelude::*;
use rome_js_syntax::JsAnyObjectMemberName;
impl FormatRule<JsAnyObjectMemberName> for FormatJsAnyObjectMemberName {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyObjectMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyObjectMemberName::JsLiteralMemberName(node) => node.format().fmt(f),
            JsAnyObjectMemberName::JsComputedMemberName(node) => node.format().fmt(f),
        }
    }
}
