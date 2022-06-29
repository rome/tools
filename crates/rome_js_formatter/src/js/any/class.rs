//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyClass;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyClass;
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
