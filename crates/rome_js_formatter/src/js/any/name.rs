//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyName;
use crate::prelude::*;
use rome_js_syntax::JsAnyName;
impl FormatRule<JsAnyName> for FormatJsAnyName {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyName::JsName(node) => node.format().fmt(f),
            JsAnyName::JsPrivateName(node) => node.format().fmt(f),
        }
    }
}
