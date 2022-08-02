//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyFunction;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyFunction;
impl FormatRule<JsAnyFunction> for FormatJsAnyFunction {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyFunction, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyFunction::JsFunctionExpression(node) => node.format().fmt(f),
            JsAnyFunction::JsFunctionDeclaration(node) => node.format().fmt(f),
            JsAnyFunction::JsArrowFunctionExpression(node) => node.format().fmt(f),
            JsAnyFunction::JsFunctionExportDefaultDeclaration(node) => node.format().fmt(f),
        }
    }
}
