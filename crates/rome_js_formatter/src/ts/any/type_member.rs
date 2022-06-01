//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyTypeMember;
use crate::prelude::*;
use rome_js_syntax::TsAnyTypeMember;
impl FormatRule<TsAnyTypeMember> for FormatTsAnyTypeMember {
    type Context = JsFormatContext;
    fn format(node: &TsAnyTypeMember, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyTypeMember::TsCallSignatureTypeMember(node) => node.format().format(f),
            TsAnyTypeMember::TsPropertySignatureTypeMember(node) => node.format().format(f),
            TsAnyTypeMember::TsConstructSignatureTypeMember(node) => node.format().format(f),
            TsAnyTypeMember::TsMethodSignatureTypeMember(node) => node.format().format(f),
            TsAnyTypeMember::TsGetterSignatureTypeMember(node) => node.format().format(f),
            TsAnyTypeMember::TsSetterSignatureTypeMember(node) => node.format().format(f),
            TsAnyTypeMember::TsIndexSignatureTypeMember(node) => node.format().format(f),
            TsAnyTypeMember::JsUnknownMember(node) => node.format().format(f),
        }
    }
}
