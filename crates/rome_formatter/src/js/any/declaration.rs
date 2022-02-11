//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyDeclaration;
impl ToFormatElement for JsAnyDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsClassDeclaration(node) => node.to_format_element(formatter),
            Self::JsFunctionDeclaration(node) => node.to_format_element(formatter),
            Self::TsEnumDeclaration(node) => node.to_format_element(formatter),
            Self::TsTypeAliasDeclaration(node) => node.to_format_element(formatter),
            Self::TsInterfaceDeclaration(node) => node.to_format_element(formatter),
            Self::JsVariableDeclaration(node) => node.to_format_element(formatter),
        }
    }
}
