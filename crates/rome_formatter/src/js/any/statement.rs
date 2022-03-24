//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyStatement;
impl ToFormatElement for JsAnyStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsBlockStatement(node) => node.to_format_element(formatter),
            Self::JsBreakStatement(node) => node.to_format_element(formatter),
            Self::JsClassDeclaration(node) => node.to_format_element(formatter),
            Self::JsContinueStatement(node) => node.to_format_element(formatter),
            Self::JsDebuggerStatement(node) => node.to_format_element(formatter),
            Self::JsDoWhileStatement(node) => node.to_format_element(formatter),
            Self::JsEmptyStatement(node) => node.to_format_element(formatter),
            Self::JsExpressionStatement(node) => node.to_format_element(formatter),
            Self::JsForInStatement(node) => node.to_format_element(formatter),
            Self::JsForOfStatement(node) => node.to_format_element(formatter),
            Self::JsForStatement(node) => node.to_format_element(formatter),
            Self::JsIfStatement(node) => node.to_format_element(formatter),
            Self::JsLabeledStatement(node) => node.to_format_element(formatter),
            Self::JsReturnStatement(node) => node.to_format_element(formatter),
            Self::JsSwitchStatement(node) => node.to_format_element(formatter),
            Self::JsThrowStatement(node) => node.to_format_element(formatter),
            Self::JsTryFinallyStatement(node) => node.to_format_element(formatter),
            Self::JsTryStatement(node) => node.to_format_element(formatter),
            Self::JsUnknownStatement(node) => node.to_format_element(formatter),
            Self::JsVariableStatement(node) => node.to_format_element(formatter),
            Self::JsWhileStatement(node) => node.to_format_element(formatter),
            Self::JsWithStatement(node) => node.to_format_element(formatter),
            Self::JsFunctionDeclaration(node) => node.to_format_element(formatter),
            Self::TsEnumDeclaration(node) => node.to_format_element(formatter),
            Self::TsTypeAliasDeclaration(node) => node.to_format_element(formatter),
            Self::TsInterfaceDeclaration(node) => node.to_format_element(formatter),
            Self::TsDeclareFunctionDeclaration(node) => node.to_format_element(formatter),
            Self::TsDeclareStatement(node) => node.to_format_element(formatter),
            Self::TsModuleDeclaration(node) => node.to_format_element(formatter),
            Self::TsExternalModuleDeclaration(node) => node.to_format_element(formatter),
            Self::TsGlobalDeclaration(node) => node.to_format_element(formatter),
            Self::TsImportEqualsDeclaration(node) => node.to_format_element(formatter),
        }
    }
}
