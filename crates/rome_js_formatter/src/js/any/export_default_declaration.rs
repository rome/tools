//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyExportDefaultDeclaration;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyExportDefaultDeclaration;
impl FormatRule<JsAnyExportDefaultDeclaration> for FormatJsAnyExportDefaultDeclaration {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyExportDefaultDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyExportDefaultDeclaration::JsClassExportDefaultDeclaration(node) => {
                node.format().fmt(f)
            }
            JsAnyExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(node) => {
                node.format().fmt(f)
            }
            JsAnyExportDefaultDeclaration::TsInterfaceDeclaration(node) => node.format().fmt(f),
            JsAnyExportDefaultDeclaration::TsDeclareFunctionExportDefaultDeclaration(node) => {
                node.format().fmt(f)
            }
        }
    }
}
