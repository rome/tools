//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::AnyJsxName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsxName;
impl FormatRule<AnyJsxName> for FormatAnyJsxName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsxName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsxName::JsxName(node) => node.format().fmt(f),
            AnyJsxName::JsxNamespaceName(node) => node.format().fmt(f),
        }
    }
}
