//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::JsAnyClassMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyClassMember;
impl FormatRule<JsAnyClassMember> for FormatJsAnyClassMember {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyClassMember::JsConstructorClassMember(node) => node.format().fmt(f),
            JsAnyClassMember::JsStaticInitializationBlockClassMember(node) => node.format().fmt(f),
            JsAnyClassMember::JsPropertyClassMember(node) => node.format().fmt(f),
            JsAnyClassMember::JsMethodClassMember(node) => node.format().fmt(f),
            JsAnyClassMember::JsGetterClassMember(node) => node.format().fmt(f),
            JsAnyClassMember::JsSetterClassMember(node) => node.format().fmt(f),
            JsAnyClassMember::TsConstructorSignatureClassMember(node) => node.format().fmt(f),
            JsAnyClassMember::TsPropertySignatureClassMember(node) => node.format().fmt(f),
            JsAnyClassMember::TsMethodSignatureClassMember(node) => node.format().fmt(f),
            JsAnyClassMember::TsGetterSignatureClassMember(node) => node.format().fmt(f),
            JsAnyClassMember::TsSetterSignatureClassMember(node) => node.format().fmt(f),
            JsAnyClassMember::TsIndexSignatureClassMember(node) => node.format().fmt(f),
            JsAnyClassMember::JsEmptyClassMember(node) => node.format().fmt(f),
            JsAnyClassMember::JsUnknownMember(node) => node.format().fmt(f),
        }
    }
}
