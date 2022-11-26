//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyDeclaration;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyDeclaration;
impl FormatRule<JsAnyDeclaration> for FormatJsAnyDeclaration {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyDeclaration::JsClassDeclaration(node) => node.format().fmt(f),
            JsAnyDeclaration::JsFunctionDeclaration(node) => node.format().fmt(f),
            JsAnyDeclaration::JsVariableDeclaration(node) => node.format().fmt(f),
            JsAnyDeclaration::TsEnumDeclaration(node) => node.format().fmt(f),
            JsAnyDeclaration::TsTypeAliasDeclaration(node) => node.format().fmt(f),
            JsAnyDeclaration::TsInterfaceDeclaration(node) => node.format().fmt(f),
            JsAnyDeclaration::TsDeclareFunctionDeclaration(node) => node.format().fmt(f),
            JsAnyDeclaration::TsModuleDeclaration(node) => node.format().fmt(f),
            JsAnyDeclaration::TsExternalModuleDeclaration(node) => node.format().fmt(f),
            JsAnyDeclaration::TsGlobalDeclaration(node) => node.format().fmt(f),
            JsAnyDeclaration::TsImportEqualsDeclaration(node) => node.format().fmt(f),
        }
    }
}
