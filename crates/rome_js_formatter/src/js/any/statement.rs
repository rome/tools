//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyStatement;
use crate::prelude::*;
use rome_js_syntax::JsAnyStatement;
impl FormatRule<JsAnyStatement> for FormatJsAnyStatement {
    type Options = JsFormatOptions;
    fn format(
        node: &JsAnyStatement,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyStatement::JsBlockStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsBreakStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsClassDeclaration(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsContinueStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsDebuggerStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsDoWhileStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsEmptyStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsExpressionStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsForInStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsForOfStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsForStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsIfStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsLabeledStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsReturnStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsSwitchStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsThrowStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsTryFinallyStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsTryStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsUnknownStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsVariableStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsWhileStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsWithStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::JsFunctionDeclaration(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::TsEnumDeclaration(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::TsTypeAliasDeclaration(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::TsInterfaceDeclaration(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::TsDeclareFunctionDeclaration(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyStatement::TsDeclareStatement(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::TsModuleDeclaration(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::TsExternalModuleDeclaration(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyStatement::TsGlobalDeclaration(node) => formatted![formatter, [node.format()]],
            JsAnyStatement::TsImportEqualsDeclaration(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
