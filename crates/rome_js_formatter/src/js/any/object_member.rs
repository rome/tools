//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyObjectMember;
use crate::prelude::*;
use rome_js_syntax::JsAnyObjectMember;
impl FormatRule<JsAnyObjectMember> for FormatJsAnyObjectMember {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyObjectMember::JsPropertyObjectMember(node) => node.format().fmt(f),
            JsAnyObjectMember::JsMethodObjectMember(node) => node.format().fmt(f),
            JsAnyObjectMember::JsGetterObjectMember(node) => node.format().fmt(f),
            JsAnyObjectMember::JsSetterObjectMember(node) => node.format().fmt(f),
            JsAnyObjectMember::JsShorthandPropertyObjectMember(node) => node.format().fmt(f),
            JsAnyObjectMember::JsSpread(node) => node.format().fmt(f),
            JsAnyObjectMember::JsUnknownMember(node) => node.format().fmt(f),
        }
    }
}
