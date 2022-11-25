//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::JsAnyDeclarationClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyDeclarationClause;
impl FormatRule<JsAnyDeclarationClause> for FormatJsAnyDeclarationClause {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyDeclarationClause, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyDeclarationClause::JsClassDeclaration(node) => node.format().fmt(f),
            JsAnyDeclarationClause::JsFunctionDeclaration(node) => node.format().fmt(f),
            JsAnyDeclarationClause::JsVariableDeclarationClause(node) => node.format().fmt(f),
            JsAnyDeclarationClause::TsEnumDeclaration(node) => node.format().fmt(f),
            JsAnyDeclarationClause::TsTypeAliasDeclaration(node) => node.format().fmt(f),
            JsAnyDeclarationClause::TsInterfaceDeclaration(node) => node.format().fmt(f),
            JsAnyDeclarationClause::TsDeclareFunctionDeclaration(node) => node.format().fmt(f),
            JsAnyDeclarationClause::TsModuleDeclaration(node) => node.format().fmt(f),
            JsAnyDeclarationClause::TsExternalModuleDeclaration(node) => node.format().fmt(f),
            JsAnyDeclarationClause::TsGlobalDeclaration(node) => node.format().fmt(f),
            JsAnyDeclarationClause::TsImportEqualsDeclaration(node) => node.format().fmt(f),
        }
    }
}
