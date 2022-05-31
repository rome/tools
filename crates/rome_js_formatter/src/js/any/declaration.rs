//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyDeclaration;
use crate::prelude::*;
use rome_js_syntax::JsAnyDeclaration;
impl FormatRule<JsAnyDeclaration> for FormatJsAnyDeclaration {
    type Context = JsFormatContext;
    fn format(node: &JsAnyDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyDeclaration::JsClassDeclaration(node) => node.format().format(f),
            JsAnyDeclaration::JsFunctionDeclaration(node) => node.format().format(f),
            JsAnyDeclaration::JsVariableDeclaration(node) => node.format().format(f),
            JsAnyDeclaration::TsEnumDeclaration(node) => node.format().format(f),
            JsAnyDeclaration::TsTypeAliasDeclaration(node) => node.format().format(f),
            JsAnyDeclaration::TsInterfaceDeclaration(node) => node.format().format(f),
            JsAnyDeclaration::TsDeclareFunctionDeclaration(node) => node.format().format(f),
            JsAnyDeclaration::TsModuleDeclaration(node) => node.format().format(f),
            JsAnyDeclaration::TsExternalModuleDeclaration(node) => node.format().format(f),
            JsAnyDeclaration::TsGlobalDeclaration(node) => node.format().format(f),
            JsAnyDeclaration::TsImportEqualsDeclaration(node) => node.format().format(f),
        }
    }
}
