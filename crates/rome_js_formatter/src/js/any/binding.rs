//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyBinding;
use crate::prelude::*;
use rome_js_syntax::JsAnyBinding;
impl FormatRule<JsAnyBinding> for FormatJsAnyBinding {
    type Options = JsFormatOptions;
    fn format(
        node: &JsAnyBinding,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyBinding::JsIdentifierBinding(node) => formatted![formatter, [node.format()]],
            JsAnyBinding::JsUnknownBinding(node) => formatted![formatter, [node.format()]],
        }
    }
}
