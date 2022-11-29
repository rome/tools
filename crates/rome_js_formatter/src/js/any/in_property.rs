//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::AnyJsInProperty;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsInProperty;
impl FormatRule<AnyJsInProperty> for FormatAnyJsInProperty {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsInProperty, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsInProperty::JsPrivateName(node) => node.format().fmt(f),
            AnyJsInProperty::AnyJsExpression(node) => node.format().fmt(f),
        }
    }
}
