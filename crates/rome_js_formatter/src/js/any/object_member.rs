//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::JsAnyObjectMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyObjectMember;
impl FormatRule<JsAnyObjectMember> for FormatJsAnyObjectMember {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
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
