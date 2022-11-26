//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyConstructorParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyConstructorParameter;
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
