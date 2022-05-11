//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyClassMember;
use crate::prelude::*;
use rome_js_syntax::JsAnyClassMember;
impl FormatRule<JsAnyClassMember> for FormatJsAnyClassMember {
    fn format(node: &JsAnyClassMember, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyClassMember::JsConstructorClassMember(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyClassMember::JsStaticInitializationBlockClassMember(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyClassMember::JsPropertyClassMember(node) => formatted![formatter, [node.format()]],
            JsAnyClassMember::JsMethodClassMember(node) => formatted![formatter, [node.format()]],
            JsAnyClassMember::JsGetterClassMember(node) => formatted![formatter, [node.format()]],
            JsAnyClassMember::JsSetterClassMember(node) => formatted![formatter, [node.format()]],
            JsAnyClassMember::TsConstructorSignatureClassMember(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyClassMember::TsPropertySignatureClassMember(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyClassMember::TsMethodSignatureClassMember(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyClassMember::TsGetterSignatureClassMember(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyClassMember::TsSetterSignatureClassMember(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyClassMember::TsIndexSignatureClassMember(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyClassMember::JsEmptyClassMember(node) => formatted![formatter, [node.format()]],
            JsAnyClassMember::JsUnknownMember(node) => formatted![formatter, [node.format()]],
        }
    }
}
