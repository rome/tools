//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyInProperty;
use crate::prelude::*;
use rome_js_syntax::JsAnyInProperty;
impl FormatRule<JsAnyInProperty> for FormatJsAnyInProperty {
    type Context = JsFormatContext;
    fn format(node: &JsAnyInProperty, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyInProperty::JsPrivateName(node) => node.format().format(f),
            JsAnyInProperty::JsAnyExpression(node) => node.format().format(f),
        }
    }
}
