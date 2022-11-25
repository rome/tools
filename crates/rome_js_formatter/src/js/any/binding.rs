//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyBinding;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyBinding;
impl FormatRule<JsAnyBinding> for FormatJsAnyBinding {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyBinding, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyBinding::JsIdentifierBinding(node) => node.format().fmt(f),
            JsAnyBinding::JsUnknownBinding(node) => node.format().fmt(f),
        }
    }
}
