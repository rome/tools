//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyConstructorParameter;
use crate::prelude::*;
use rome_js_syntax::JsAnyConstructorParameter;
impl FormatRule<JsAnyConstructorParameter> for FormatJsAnyConstructorParameter {
    fn format(
        node: &JsAnyConstructorParameter,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyConstructorParameter::JsAnyFormalParameter(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyConstructorParameter::JsRestParameter(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyConstructorParameter::TsPropertyParameter(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
