//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyStatement;
impl ToFormatElement for JsAnyStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsBlockStatement(node) => node.format(formatter),
            Self::JsBreakStatement(node) => node.format(formatter),
            Self::JsClassDeclaration(node) => node.format(formatter),
            Self::JsContinueStatement(node) => node.format(formatter),
            Self::JsDebuggerStatement(node) => node.format(formatter),
            Self::JsDoWhileStatement(node) => node.format(formatter),
            Self::JsEmptyStatement(node) => node.format(formatter),
            Self::JsExpressionStatement(node) => node.format(formatter),
            Self::JsForInStatement(node) => node.format(formatter),
            Self::JsForOfStatement(node) => node.format(formatter),
            Self::JsForStatement(node) => node.format(formatter),
            Self::JsIfStatement(node) => node.format(formatter),
            Self::JsLabeledStatement(node) => node.format(formatter),
            Self::JsReturnStatement(node) => node.format(formatter),
            Self::JsSwitchStatement(node) => node.format(formatter),
            Self::JsThrowStatement(node) => node.format(formatter),
            Self::JsTryFinallyStatement(node) => node.format(formatter),
            Self::JsTryStatement(node) => node.format(formatter),
            Self::JsUnknownStatement(node) => node.format(formatter),
            Self::JsVariableStatement(node) => node.format(formatter),
            Self::JsWhileStatement(node) => node.format(formatter),
            Self::JsWithStatement(node) => node.format(formatter),
            Self::JsFunctionDeclaration(node) => node.format(formatter),
            Self::TsEnumDeclaration(node) => node.format(formatter),
            Self::TsTypeAliasDeclaration(node) => node.format(formatter),
            Self::TsInterfaceDeclaration(node) => node.format(formatter),
            Self::TsDeclareFunctionDeclaration(node) => node.format(formatter),
            Self::TsDeclareStatement(node) => node.format(formatter),
            Self::TsModuleDeclaration(node) => node.format(formatter),
            Self::TsExternalModuleDeclaration(node) => node.format(formatter),
            Self::TsGlobalDeclaration(node) => node.format(formatter),
            Self::TsImportEqualsDeclaration(node) => node.format(formatter),
        }
    }
}
