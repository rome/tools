//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyInProperty;
use crate::prelude::*;
use rome_js_syntax::JsAnyInProperty;
impl FormatRule<JsAnyInProperty> for FormatJsAnyInProperty {
    type Context = JsFormatContext;
    fn format(node: &JsAnyInProperty, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyInProperty::JsPrivateName(node) => formatted![formatter, [node.format()]],
            JsAnyInProperty::JsAnyExpression(node) => formatted![formatter, [node.format()]],
        }
    }
}
