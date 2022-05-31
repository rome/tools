//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyArrowFunctionParameters;
use crate::prelude::*;
use rome_js_syntax::JsAnyArrowFunctionParameters;
impl FormatRule<JsAnyArrowFunctionParameters> for FormatJsAnyArrowFunctionParameters {
    type Context = JsFormatContext;
    fn format(node: &JsAnyArrowFunctionParameters, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyArrowFunctionParameters::JsParameters(node) => node.format().format(f),
            JsAnyArrowFunctionParameters::JsAnyBinding(node) => node.format().format(f),
        }
    }
}
