//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyArrowFunctionParameters;
use crate::prelude::*;
use rome_js_syntax::JsAnyArrowFunctionParameters;
impl FormatRule<JsAnyArrowFunctionParameters> for FormatJsAnyArrowFunctionParameters {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyArrowFunctionParameters, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyArrowFunctionParameters::JsParameters(node) => node.format().fmt(f),
            JsAnyArrowFunctionParameters::JsAnyBinding(node) => node.format().fmt(f),
        }
    }
}
