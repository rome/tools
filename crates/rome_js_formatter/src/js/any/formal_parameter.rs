//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyFormalParameter;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyFormalParameter;
impl FormatRule<JsAnyFormalParameter> for FormatJsAnyFormalParameter {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyFormalParameter, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyFormalParameter::JsFormalParameter(node) => node.format().fmt(f),
            JsAnyFormalParameter::JsUnknownParameter(node) => node.format().fmt(f),
        }
    }
}
