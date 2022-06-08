//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyStatement;
use crate::prelude::*;
use rome_js_syntax::JsAnyStatement;
impl FormatRule<JsAnyStatement> for FormatJsAnyStatement {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyStatement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyStatement::JsBlockStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsBreakStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsClassDeclaration(node) => node.format().fmt(f),
            JsAnyStatement::JsContinueStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsDebuggerStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsDoWhileStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsEmptyStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsExpressionStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsForInStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsForOfStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsForStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsIfStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsLabeledStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsReturnStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsSwitchStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsThrowStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsTryFinallyStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsTryStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsUnknownStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsVariableStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsWhileStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsWithStatement(node) => node.format().fmt(f),
            JsAnyStatement::JsFunctionDeclaration(node) => node.format().fmt(f),
            JsAnyStatement::TsEnumDeclaration(node) => node.format().fmt(f),
            JsAnyStatement::TsTypeAliasDeclaration(node) => node.format().fmt(f),
            JsAnyStatement::TsInterfaceDeclaration(node) => node.format().fmt(f),
            JsAnyStatement::TsDeclareFunctionDeclaration(node) => node.format().fmt(f),
            JsAnyStatement::TsDeclareStatement(node) => node.format().fmt(f),
            JsAnyStatement::TsModuleDeclaration(node) => node.format().fmt(f),
            JsAnyStatement::TsExternalModuleDeclaration(node) => node.format().fmt(f),
            JsAnyStatement::TsGlobalDeclaration(node) => node.format().fmt(f),
            JsAnyStatement::TsImportEqualsDeclaration(node) => node.format().fmt(f),
        }
    }
}
