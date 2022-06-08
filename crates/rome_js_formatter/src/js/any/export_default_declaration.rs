//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyExportDefaultDeclaration;
use crate::prelude::*;
use rome_js_syntax::JsAnyExportDefaultDeclaration;
impl FormatRule<JsAnyExportDefaultDeclaration> for FormatJsAnyExportDefaultDeclaration {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyExportDefaultDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyExportDefaultDeclaration::JsClassExportDefaultDeclaration(node) => {
                node.format().fmt(f)
            }
            JsAnyExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(node) => {
                node.format().fmt(f)
            }
            JsAnyExportDefaultDeclaration::TsDeclareFunctionDeclaration(node) => {
                node.format().fmt(f)
            }
            JsAnyExportDefaultDeclaration::TsInterfaceDeclaration(node) => node.format().fmt(f),
        }
    }
}
