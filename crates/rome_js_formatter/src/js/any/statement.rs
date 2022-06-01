//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyStatement;
use crate::prelude::*;
use rome_js_syntax::JsAnyStatement;
impl FormatRule<JsAnyStatement> for FormatJsAnyStatement {
    type Context = JsFormatContext;
    fn format(node: &JsAnyStatement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyStatement::JsBlockStatement(node) => node.format().format(f),
            JsAnyStatement::JsBreakStatement(node) => node.format().format(f),
            JsAnyStatement::JsClassDeclaration(node) => node.format().format(f),
            JsAnyStatement::JsContinueStatement(node) => node.format().format(f),
            JsAnyStatement::JsDebuggerStatement(node) => node.format().format(f),
            JsAnyStatement::JsDoWhileStatement(node) => node.format().format(f),
            JsAnyStatement::JsEmptyStatement(node) => node.format().format(f),
            JsAnyStatement::JsExpressionStatement(node) => node.format().format(f),
            JsAnyStatement::JsForInStatement(node) => node.format().format(f),
            JsAnyStatement::JsForOfStatement(node) => node.format().format(f),
            JsAnyStatement::JsForStatement(node) => node.format().format(f),
            JsAnyStatement::JsIfStatement(node) => node.format().format(f),
            JsAnyStatement::JsLabeledStatement(node) => node.format().format(f),
            JsAnyStatement::JsReturnStatement(node) => node.format().format(f),
            JsAnyStatement::JsSwitchStatement(node) => node.format().format(f),
            JsAnyStatement::JsThrowStatement(node) => node.format().format(f),
            JsAnyStatement::JsTryFinallyStatement(node) => node.format().format(f),
            JsAnyStatement::JsTryStatement(node) => node.format().format(f),
            JsAnyStatement::JsUnknownStatement(node) => node.format().format(f),
            JsAnyStatement::JsVariableStatement(node) => node.format().format(f),
            JsAnyStatement::JsWhileStatement(node) => node.format().format(f),
            JsAnyStatement::JsWithStatement(node) => node.format().format(f),
            JsAnyStatement::JsFunctionDeclaration(node) => node.format().format(f),
            JsAnyStatement::TsEnumDeclaration(node) => node.format().format(f),
            JsAnyStatement::TsTypeAliasDeclaration(node) => node.format().format(f),
            JsAnyStatement::TsInterfaceDeclaration(node) => node.format().format(f),
            JsAnyStatement::TsDeclareFunctionDeclaration(node) => node.format().format(f),
            JsAnyStatement::TsDeclareStatement(node) => node.format().format(f),
            JsAnyStatement::TsModuleDeclaration(node) => node.format().format(f),
            JsAnyStatement::TsExternalModuleDeclaration(node) => node.format().format(f),
            JsAnyStatement::TsGlobalDeclaration(node) => node.format().format(f),
            JsAnyStatement::TsImportEqualsDeclaration(node) => node.format().format(f),
        }
    }
}
