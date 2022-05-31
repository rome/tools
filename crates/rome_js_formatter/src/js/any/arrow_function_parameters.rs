//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyArrowFunctionParameters;
use crate::prelude::*;
use rome_js_syntax::JsAnyArrowFunctionParameters;
impl FormatRule<JsAnyArrowFunctionParameters> for FormatJsAnyArrowFunctionParameters {
    type Context = JsFormatContext;
    fn format(
        node: &JsAnyArrowFunctionParameters,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyArrowFunctionParameters::JsParameters(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyArrowFunctionParameters::JsAnyBinding(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
