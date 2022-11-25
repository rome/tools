//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::JsAnyArrowFunctionParameters;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyArrowFunctionParameters;
impl FormatRule<JsAnyArrowFunctionParameters> for FormatJsAnyArrowFunctionParameters {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyArrowFunctionParameters, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyArrowFunctionParameters::JsParameters(node) => node.format().fmt(f),
            JsAnyArrowFunctionParameters::JsAnyBinding(node) => node.format().fmt(f),
        }
    }
}
