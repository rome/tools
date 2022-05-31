//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyClassMember;
use crate::prelude::*;
use rome_js_syntax::JsAnyClassMember;
impl FormatRule<JsAnyClassMember> for FormatJsAnyClassMember {
    type Context = JsFormatContext;
    fn format(node: &JsAnyClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyClassMember::JsConstructorClassMember(node) => node.format().format(f),
            JsAnyClassMember::JsStaticInitializationBlockClassMember(node) => {
                node.format().format(f)
            }
            JsAnyClassMember::JsPropertyClassMember(node) => node.format().format(f),
            JsAnyClassMember::JsMethodClassMember(node) => node.format().format(f),
            JsAnyClassMember::JsGetterClassMember(node) => node.format().format(f),
            JsAnyClassMember::JsSetterClassMember(node) => node.format().format(f),
            JsAnyClassMember::TsConstructorSignatureClassMember(node) => node.format().format(f),
            JsAnyClassMember::TsPropertySignatureClassMember(node) => node.format().format(f),
            JsAnyClassMember::TsMethodSignatureClassMember(node) => node.format().format(f),
            JsAnyClassMember::TsGetterSignatureClassMember(node) => node.format().format(f),
            JsAnyClassMember::TsSetterSignatureClassMember(node) => node.format().format(f),
            JsAnyClassMember::TsIndexSignatureClassMember(node) => node.format().format(f),
            JsAnyClassMember::JsEmptyClassMember(node) => node.format().format(f),
            JsAnyClassMember::JsUnknownMember(node) => node.format().format(f),
        }
    }
}
