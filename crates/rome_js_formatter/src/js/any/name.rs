//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyName;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyName;
impl FormatRule<JsAnyName> for FormatJsAnyName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyName::JsName(node) => node.format().fmt(f),
            JsAnyName::JsPrivateName(node) => node.format().fmt(f),
        }
    }
}
