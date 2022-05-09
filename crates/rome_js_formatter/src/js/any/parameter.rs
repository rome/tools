//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyParameter;
use crate::prelude::*;
use rome_js_syntax::JsAnyParameter;
impl FormatRule<JsAnyParameter> for FormatJsAnyParameter {
    fn format(node: &JsAnyParameter, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyParameter::JsAnyFormalParameter(node) => formatted![formatter, node.format()],
            JsAnyParameter::JsRestParameter(node) => formatted![formatter, node.format()],
            JsAnyParameter::TsThisParameter(node) => formatted![formatter, node.format()],
        }
    }
}
