//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyTypeMember;
use crate::prelude::*;
use rome_js_syntax::TsAnyTypeMember;
impl FormatRule<TsAnyTypeMember> for FormatTsAnyTypeMember {
    type Options = JsFormatOptions;
    fn format(
        node: &TsAnyTypeMember,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            TsAnyTypeMember::TsCallSignatureTypeMember(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyTypeMember::TsPropertySignatureTypeMember(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyTypeMember::TsConstructSignatureTypeMember(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyTypeMember::TsMethodSignatureTypeMember(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyTypeMember::TsGetterSignatureTypeMember(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyTypeMember::TsSetterSignatureTypeMember(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyTypeMember::TsIndexSignatureTypeMember(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyTypeMember::JsUnknownMember(node) => formatted![formatter, [node.format()]],
        }
    }
}
