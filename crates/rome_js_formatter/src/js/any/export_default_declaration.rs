//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyExportDefaultDeclaration;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyExportDefaultDeclaration;
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
