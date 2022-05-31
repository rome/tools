//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyDeclarationClause;
use crate::prelude::*;
use rome_js_syntax::JsAnyDeclarationClause;
impl FormatRule<JsAnyDeclarationClause> for FormatJsAnyDeclarationClause {
    type Context = JsFormatContext;
    fn format(node: &JsAnyDeclarationClause, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyDeclarationClause::JsClassDeclaration(node) => node.format().format(f),
            JsAnyDeclarationClause::JsFunctionDeclaration(node) => node.format().format(f),
            JsAnyDeclarationClause::JsVariableDeclarationClause(node) => node.format().format(f),
            JsAnyDeclarationClause::TsEnumDeclaration(node) => node.format().format(f),
            JsAnyDeclarationClause::TsTypeAliasDeclaration(node) => node.format().format(f),
            JsAnyDeclarationClause::TsInterfaceDeclaration(node) => node.format().format(f),
            JsAnyDeclarationClause::TsDeclareFunctionDeclaration(node) => node.format().format(f),
            JsAnyDeclarationClause::TsModuleDeclaration(node) => node.format().format(f),
            JsAnyDeclarationClause::TsExternalModuleDeclaration(node) => node.format().format(f),
            JsAnyDeclarationClause::TsGlobalDeclaration(node) => node.format().format(f),
            JsAnyDeclarationClause::TsImportEqualsDeclaration(node) => node.format().format(f),
        }
    }
}
