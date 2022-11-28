//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::AnyJsBinding;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsBinding;
impl FormatRule<AnyJsBinding> for FormatAnyJsBinding {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsBinding, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsBinding::JsIdentifierBinding(node) => node.format().fmt(f),
            AnyJsBinding::JsBogusBinding(node) => node.format().fmt(f),
        }
    }
}
