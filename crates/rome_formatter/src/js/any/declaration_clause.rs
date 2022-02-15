//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyDeclarationClause;
impl ToFormatElement for JsAnyDeclarationClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsClassDeclaration(node) => node.to_format_element(formatter),
            Self::JsFunctionDeclaration(node) => node.to_format_element(formatter),
            Self::JsVariableDeclarationClause(node) => node.to_format_element(formatter),
            Self::TsEnumDeclaration(node) => node.to_format_element(formatter),
            Self::TsTypeAliasDeclaration(node) => node.to_format_element(formatter),
            Self::TsInterfaceDeclaration(node) => node.to_format_element(formatter),
            Self::TsDeclareFunctionDeclaration(node) => node.to_format_element(formatter),
            Self::TsModuleDeclaration(node) => node.to_format_element(formatter),
            Self::TsExternalModuleDeclaration(node) => node.to_format_element(formatter),
            Self::TsGlobalDeclaration(node) => node.to_format_element(formatter),
            Self::TsImportEqualsDeclaration(node) => node.to_format_element(formatter),
        }
    }
}
