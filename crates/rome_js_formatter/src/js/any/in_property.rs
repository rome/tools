//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyInProperty;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyInProperty;
impl FormatRule<JsAnyInProperty> for FormatJsAnyInProperty {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyInProperty, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyInProperty::JsPrivateName(node) => node.format().fmt(f),
            JsAnyInProperty::JsAnyExpression(node) => node.format().fmt(f),
        }
    }
}
