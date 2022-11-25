//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyClass;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyClass;
impl FormatRule<JsAnyClass> for FormatJsAnyClass {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyClass, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyClass::JsClassDeclaration(node) => node.format().fmt(f),
            JsAnyClass::JsClassExpression(node) => node.format().fmt(f),
            JsAnyClass::JsClassExportDefaultDeclaration(node) => node.format().fmt(f),
        }
    }
}
