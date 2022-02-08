//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyStatement;
impl ToFormatElement for JsAnyStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsBlockStatement(node) => node.to_format_element(formatter),
            Self::JsBreakStatement(node) => node.to_format_element(formatter),
            Self::JsClassStatement(node) => node.to_format_element(formatter),
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
            Self::TsEnumStatement(node) => node.to_format_element(formatter),
            Self::JsFunctionStatement(node) => node.to_format_element(formatter),
            Self::TsTypeAliasStatement(node) => node.to_format_element(formatter),
        }
    }
}
