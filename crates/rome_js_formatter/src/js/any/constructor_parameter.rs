//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyConstructorParameter;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyConstructorParameter;
impl FormatRule<JsAnyConstructorParameter> for FormatJsAnyConstructorParameter {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyConstructorParameter, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyConstructorParameter::JsAnyFormalParameter(node) => node.format().fmt(f),
            JsAnyConstructorParameter::JsRestParameter(node) => node.format().fmt(f),
            JsAnyConstructorParameter::TsPropertyParameter(node) => node.format().fmt(f),
        }
    }
}
