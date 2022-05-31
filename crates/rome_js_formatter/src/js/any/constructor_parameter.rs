//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyConstructorParameter;
use crate::prelude::*;
use rome_js_syntax::JsAnyConstructorParameter;
impl FormatRule<JsAnyConstructorParameter> for FormatJsAnyConstructorParameter {
    type Context = JsFormatContext;
    fn format(node: &JsAnyConstructorParameter, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyConstructorParameter::JsAnyFormalParameter(node) => node.format().format(f),
            JsAnyConstructorParameter::JsRestParameter(node) => node.format().format(f),
            JsAnyConstructorParameter::TsPropertyParameter(node) => node.format().format(f),
        }
    }
}
