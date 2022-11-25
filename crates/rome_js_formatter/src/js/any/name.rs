//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::JsAnyName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyName;
impl FormatRule<JsAnyName> for FormatJsAnyName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyName::JsName(node) => node.format().fmt(f),
            JsAnyName::JsPrivateName(node) => node.format().fmt(f),
        }
    }
}
