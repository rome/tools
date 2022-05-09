//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyFormalParameter;
use crate::prelude::*;
use rome_js_syntax::JsAnyFormalParameter;
impl FormatRule<JsAnyFormalParameter> for FormatJsAnyFormalParameter {
    fn format(node: &JsAnyFormalParameter, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyFormalParameter::JsFormalParameter(node) => formatted![formatter, node.format()],
            JsAnyFormalParameter::JsUnknownParameter(node) => formatted![formatter, node.format()],
        }
    }
}
