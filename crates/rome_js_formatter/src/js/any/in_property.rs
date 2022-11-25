//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyInProperty;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyInProperty;
impl FormatRule<JsAnyInProperty> for FormatJsAnyInProperty {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyInProperty, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyInProperty::JsPrivateName(node) => node.format().fmt(f),
            JsAnyInProperty::JsAnyExpression(node) => node.format().fmt(f),
        }
    }
}
