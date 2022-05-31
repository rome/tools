//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyObjectMember;
use crate::prelude::*;
use rome_js_syntax::JsAnyObjectMember;
impl FormatRule<JsAnyObjectMember> for FormatJsAnyObjectMember {
    type Context = JsFormatContext;
    fn format(node: &JsAnyObjectMember, f: &mut Formatter<Self::Context>) -> FormatResult<()> {
        match node {
            JsAnyObjectMember::JsPropertyObjectMember(node) => node.format().format(f),
            JsAnyObjectMember::JsMethodObjectMember(node) => node.format().format(f),
            JsAnyObjectMember::JsGetterObjectMember(node) => node.format().format(f),
            JsAnyObjectMember::JsSetterObjectMember(node) => node.format().format(f),
            JsAnyObjectMember::JsShorthandPropertyObjectMember(node) => node.format().format(f),
            JsAnyObjectMember::JsSpread(node) => node.format().format(f),
            JsAnyObjectMember::JsUnknownMember(node) => node.format().format(f),
        }
    }
}
