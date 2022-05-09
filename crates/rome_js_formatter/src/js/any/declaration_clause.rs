//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyDeclarationClause;
use crate::prelude::*;
use rome_js_syntax::JsAnyDeclarationClause;
impl FormatRule<JsAnyDeclarationClause> for FormatJsAnyDeclarationClause {
    fn format(node: &JsAnyDeclarationClause, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyDeclarationClause::JsClassDeclaration(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyDeclarationClause::JsFunctionDeclaration(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyDeclarationClause::JsVariableDeclarationClause(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyDeclarationClause::TsEnumDeclaration(node) => formatted![formatter, node.format()],
            JsAnyDeclarationClause::TsTypeAliasDeclaration(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyDeclarationClause::TsInterfaceDeclaration(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyDeclarationClause::TsDeclareFunctionDeclaration(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyDeclarationClause::TsModuleDeclaration(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyDeclarationClause::TsExternalModuleDeclaration(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyDeclarationClause::TsGlobalDeclaration(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyDeclarationClause::TsImportEqualsDeclaration(node) => {
                formatted![formatter, node.format()]
            }
        }
    }
}
