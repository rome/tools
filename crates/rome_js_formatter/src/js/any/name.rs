//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyName;
use crate::prelude::*;
use rome_js_syntax::JsAnyName;
impl FormatRule<JsAnyName> for FormatJsAnyName {
    type Context = JsFormatContext;
    fn format(
        node: &JsAnyName,
        formatter: &Formatter<Self::Context>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyName::JsName(node) => formatted![formatter, [node.format()]],
            JsAnyName::JsPrivateName(node) => formatted![formatter, [node.format()]],
        }
    }
}
