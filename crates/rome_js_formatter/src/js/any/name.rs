//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyName;
use crate::prelude::*;
use rome_js_syntax::JsAnyName;
impl FormatRule<JsAnyName> for FormatJsAnyName {
    type Context = JsFormatContext;
    fn format(node: &JsAnyName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyName::JsName(node) => node.format().format(f),
            JsAnyName::JsPrivateName(node) => node.format().format(f),
        }
    }
}
