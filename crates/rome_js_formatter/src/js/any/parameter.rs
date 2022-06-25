//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyParameter;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyParameter;
impl FormatRule<JsAnyParameter> for FormatJsAnyParameter {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyParameter, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyParameter::JsAnyFormalParameter(node) => node.format().fmt(f),
            JsAnyParameter::JsRestParameter(node) => node.format().fmt(f),
            JsAnyParameter::TsThisParameter(node) => node.format().fmt(f),
        }
    }
}
