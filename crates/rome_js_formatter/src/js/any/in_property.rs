//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyInProperty;
use crate::prelude::*;
use rome_js_syntax::JsAnyInProperty;
impl FormatRule<JsAnyInProperty> for FormatJsAnyInProperty {
    type Options = JsFormatOptions;
    fn format(
        node: &JsAnyInProperty,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyInProperty::JsPrivateName(node) => formatted![formatter, [node.format()]],
            JsAnyInProperty::JsAnyExpression(node) => formatted![formatter, [node.format()]],
        }
    }
}
