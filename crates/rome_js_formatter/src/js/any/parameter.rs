//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyParameter;
use crate::prelude::*;
use rome_js_syntax::JsAnyParameter;
impl FormatRule<JsAnyParameter> for FormatJsAnyParameter {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyParameter, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyParameter::JsAnyFormalParameter(node) => node.format().fmt(f),
            JsAnyParameter::JsRestParameter(node) => node.format().fmt(f),
            JsAnyParameter::TsThisParameter(node) => node.format().fmt(f),
        }
    }
}
