//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyFormalParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyFormalParameter;
impl FormatRule<JsAnyFormalParameter> for FormatJsAnyFormalParameter {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyFormalParameter, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyFormalParameter::JsFormalParameter(node) => node.format().fmt(f),
            JsAnyFormalParameter::JsBogusParameter(node) => node.format().fmt(f),
        }
    }
}
