//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyBinding;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyBinding;
impl FormatRule<JsAnyBinding> for FormatJsAnyBinding {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyBinding, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyBinding::JsIdentifierBinding(node) => node.format().fmt(f),
            JsAnyBinding::JsUnknownBinding(node) => node.format().fmt(f),
        }
    }
}
