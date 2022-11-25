//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyParameter;
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
