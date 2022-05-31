//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyFormalParameter;
use crate::prelude::*;
use rome_js_syntax::JsAnyFormalParameter;
impl FormatRule<JsAnyFormalParameter> for FormatJsAnyFormalParameter {
    type Context = JsFormatContext;
    fn format(node: &JsAnyFormalParameter, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyFormalParameter::JsFormalParameter(node) => node.format().format(f),
            JsAnyFormalParameter::JsUnknownParameter(node) => node.format().format(f),
        }
    }
}
