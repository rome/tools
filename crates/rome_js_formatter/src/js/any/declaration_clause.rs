//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyDeclarationClause;
use crate::prelude::*;
use rome_js_syntax::JsAnyDeclarationClause;
impl FormatRule<JsAnyDeclarationClause> for FormatJsAnyDeclarationClause {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyDeclarationClause, f: &mut JsFormatter) -> FormatResult<()> {
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
